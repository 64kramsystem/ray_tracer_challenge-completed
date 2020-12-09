use std::{
    collections::HashMap,
    error::Error,
    io::{self, BufRead, BufReader},
    sync::Arc,
};

use regex::Regex;

use crate::{
    math::Tuple,
    space::{Group, Shape, Triangle},
};

use ParsedElement::*;

lazy_static::lazy_static! {
    // Faces with texture vertices (`f a/b/c...`) are decoded as faces with normals only, as texture
    // vertices are not supported.

    static ref VERTEX_REGEX: Regex = Regex::new(r"^v (-?\d(?:\.\d+)?) (-?\d(?:\.\d+)?) (-?\d(?:\.\d+)?)$").unwrap();
    static ref VERTEX_NORMAL_REGEX: Regex = Regex::new(r"^vn (-?\d(?:\.\d+)?) (-?\d(?:\.\d+)?) (-?\d(?:\.\d+)?)$").unwrap();
    static ref FACES_REGEX: Regex = Regex::new(r"^f (\d+) (\d+(?: \d+)+)$").unwrap();
    static ref FACE_WITH_NORMAL_REGEX: Regex = Regex::new(r"^f (\d+)/\d*/(\d+) (\d+)/\d*/(\d+) (\d+)/\d*/(\d+)$").unwrap();
    static ref GROUP_REGEX: Regex = Regex::new(r"^g (\w+)$").unwrap();
}

// The book doesn't actually clarify what happens to the default group once group definitions parsing
// is introduced.
//
const DEFAULT_GROUP_NAME: &str = "default";

enum ParsedElement {
    Vertex(Tuple),
    VertexNormal(Tuple),
    Faces(Vec<(usize, usize, usize)>),
    FaceWithNormal((usize, usize), (usize, usize), (usize, usize)),
    Group(String),
    Invalid,
}

pub struct ObjParser {
    // WATCH OUT!!! DON'T ACCESS VERTICES/NORMALS DIRECTLY, WHILE PARSING!!!
    // The indexes are 1-based, which are extremely easy to mistake.
    //
    vertices: Vec<Tuple>,
    normals: Vec<Tuple>,
    groups: HashMap<String, Arc<dyn Shape>>,
}

impl ObjParser {
    pub fn parse<T: io::Read>(reader: T) -> Result<Self, Box<dyn Error>> {
        let reader = BufReader::new(reader);

        // Ownership is a bit tricky. It's not possible to use borrowed keys, because are inside the
        // for loop (match) scope, they don't survive this (the outer) scope.
        //
        let mut groups = HashMap::new();
        let default_group: Arc<dyn Shape> = Arc::new(Group::default());
        groups.insert(DEFAULT_GROUP_NAME.to_string(), default_group);

        let mut parser = Self {
            vertices: vec![],
            normals: vec![],
            groups,
        };

        let mut current_group_name = DEFAULT_GROUP_NAME.to_string();

        for line in reader.lines() {
            let parsed_element = Self::parse_line(line?);

            match parsed_element {
                Vertex(vertex) => parser.vertices.push(vertex),
                VertexNormal(normal) => parser.normals.push(normal),
                Faces(vertex_indexes) => {
                    for (p1i, p2i, p3i) in vertex_indexes {
                        let p1 = parser.vertex(p1i);
                        let p2 = parser.vertex(p2i);
                        let p3 = parser.vertex(p3i);

                        let triangle: Arc<dyn Shape> = Arc::new(Triangle::new(p1, p2, p3));

                        let group = parser.groups.entry(current_group_name.to_string());
                        group.and_modify(|group| Group::add_child(group, &triangle));
                    }
                }
                FaceWithNormal((p1i, n1i), (p2i, n2i), (p3i, n3i)) => {
                    let p1 = parser.vertex(p1i);
                    let p2 = parser.vertex(p2i);
                    let p3 = parser.vertex(p3i);
                    let n1 = parser.normal(n1i);
                    let n2 = parser.normal(n2i);
                    let n3 = parser.normal(n3i);

                    let triangle: Arc<dyn Shape> =
                        Arc::new(Triangle::smooth(p1, p2, p3, n1, n2, n3));

                    let group = parser.groups.entry(current_group_name.to_string());
                    group.and_modify(|group| Group::add_child(group, &triangle));
                }
                Group(group_name) => {
                    let groups = &mut parser.groups;
                    groups
                        .entry(group_name.clone())
                        .or_insert(Arc::new(Group::default()));
                    current_group_name = group_name;
                }
                Invalid => {}
            }
        }

        Ok(parser)
    }

    pub fn default_group(&self) -> &Arc<dyn Shape> {
        &self.groups[DEFAULT_GROUP_NAME]
    }

    pub fn group(&self, group_name: &str) -> Arc<dyn Shape> {
        Arc::clone(&self.groups[group_name])
    }

    // Export the groups as tree, with thre group as leaves of a new root group.
    // In the group, this is `obj_to_group()`;
    //
    pub fn export_tree(&self) -> Arc<dyn Shape> {
        let root_group: Arc<dyn Shape> = Arc::new(Group::default());

        for group in self.groups.values() {
            Group::add_child(&root_group, &group)
        }

        root_group
    }

    pub fn vertex(&self, i: usize) -> Tuple {
        self.vertices[i - 1]
    }

    pub fn normal(&self, i: usize) -> Tuple {
        self.normals[i - 1]
    }

    fn parse_line(line: String) -> ParsedElement {
        if let Some(captures) = VERTEX_REGEX.captures(&line) {
            let x: f64 = captures[1].parse().unwrap();
            let y: f64 = captures[2].parse().unwrap();
            let z: f64 = captures[3].parse().unwrap();

            ParsedElement::Vertex(Tuple::point(x, y, z))
        } else if let Some(captures) = VERTEX_NORMAL_REGEX.captures(&line) {
            let x: f64 = captures[1].parse().unwrap();
            let y: f64 = captures[2].parse().unwrap();
            let z: f64 = captures[3].parse().unwrap();

            ParsedElement::VertexNormal(Tuple::vector(x, y, z))
        } else if let Some(captures) = FACES_REGEX.captures(&line) {
            let mut faces = vec![];

            let p1i: usize = captures[1].parse().unwrap();

            let all_other_ps_i = captures[2]
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            for other_ps_i in all_other_ps_i.windows(2) {
                let p2i = other_ps_i[0];
                let p3i = other_ps_i[1];

                faces.push((p1i, p2i, p3i));
            }

            ParsedElement::Faces(faces)
        } else if let Some(captures) = FACE_WITH_NORMAL_REGEX.captures(&line) {
            // MWAHAHAHA

            let values = captures
                .iter()
                .skip(1)
                .map(|c| c.unwrap().as_str())
                .map(|c| c.parse().unwrap())
                .collect::<Vec<_>>();

            if let [v1, n1, v2, n2, v3, n3] = values.as_slice() {
                FaceWithNormal((*v1, *n1), (*v2, *n2), (*v3, *n3))
            } else {
                unreachable!()
            }
        } else if let Some(captures) = GROUP_REGEX.captures(&line) {
            let name = captures[1].to_string();

            ParsedElement::Group(name)
        } else {
            Invalid
        }
    }
}
