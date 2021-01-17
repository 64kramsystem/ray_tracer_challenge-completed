use std::{
    collections::HashMap,
    error::Error,
    io::{self, BufRead, BufReader},
    sync::Arc,
};

use regex::Regex;

use crate::{
    math::{Matrix, Tuple},
    space::{Group, Shape, Triangle},
};

use ParsedElement::*;

lazy_static::lazy_static! {
    // Faces with texture vertices (`f a/b/c...`) are decoded as faces with normals only, as texture
    // vertices are not supported.
    // The "Faces with texture" regex could be merged into the bare "Faces" one, but it gets too messy.

    static ref VERTEX_REGEX: Regex = Regex::new(r"^v (-?\d+(?:\.\d+)?) (-?\d+(?:\.\d+)?) (-?\d+(?:\.\d+)?)$").unwrap();
    static ref VERTEX_NORMAL_REGEX: Regex = Regex::new(r"^vn (-?\d+(?:\.\d+)?) (-?\d+(?:\.\d+)?) (-?\d+(?:\.\d+)?)$").unwrap();
    static ref FACES_REGEX: Regex = Regex::new(r"^f (\d+) (\d+(?: \d+)+)$").unwrap();
    static ref FACES_WITH_TEXTURE_REGEX: Regex = Regex::new(r"^f (\d+)/\d*/ (\d+)/\d*/ (\d+)/\d*/$").unwrap();
    static ref FACE_WITH_NORMAL_REGEX: Regex = Regex::new(r"^f (\d+)/\d*/(\d+) (\d+)/\d*/(\d+) (\d+)/\d*/(\d+)$").unwrap();
    static ref GROUP_REGEX: Regex = Regex::new(r"^g (\w+)$").unwrap();
}

// The book doesn't actually clarify what happens to the default group once group definitions parsing
// is introduced.
//
const DEFAULT_GROUP_NAME: &str = "default";

#[derive(Debug)]
enum ParsedElement {
    Vertex(Tuple),
    VertexNormal(Tuple),
    Faces(Vec<(usize, usize, usize)>),
    Face(usize, usize, usize),
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
    // The values are only Face/FaceWithNormal.
    //
    groups_data: HashMap<String, Vec<ParsedElement>>,
}

impl ObjParser {
    pub fn parse<T: io::Read>(reader: T) -> Result<Self, Box<dyn Error>> {
        let reader = BufReader::new(reader);

        // Ownership is a bit tricky. It's not possible to use borrowed keys, because are inside the
        // for loop (match) scope, they don't survive this (the outer) scope.
        //
        let mut groups = HashMap::new();

        groups.insert(DEFAULT_GROUP_NAME.to_string(), vec![]);

        let mut parser = Self {
            vertices: vec![],
            normals: vec![],
            groups_data: groups,
        };

        let mut current_group_name = DEFAULT_GROUP_NAME.to_string();

        for line in reader.lines() {
            let parsed_element = Self::parse_line(line?);

            match parsed_element {
                Vertex(vertex) => parser.vertices.push(vertex),
                VertexNormal(normal) => parser.normals.push(normal),
                Faces(vertex_indexes) => {
                    for (p1i, p2i, p3i) in vertex_indexes {
                        let group = parser.groups_data.entry(current_group_name.to_string());
                        group.and_modify(|group| group.push(Face(p1i, p2i, p3i)));
                    }
                }
                Face(p1i, p2i, p3i) => {
                    let group = parser.groups_data.entry(current_group_name.to_string());
                    group.and_modify(|group| group.push(Face(p1i, p2i, p3i)));
                }
                FaceWithNormal(_, _, _) => {
                    let group = parser.groups_data.entry(current_group_name.to_string());
                    group.and_modify(|group| group.push(parsed_element));
                }
                Group(group_name) => {
                    let groups = &mut parser.groups_data;
                    groups.entry(group_name.clone()).or_insert_with(Vec::new);
                    current_group_name = group_name;
                }
                Invalid => {}
            }
        }

        Ok(parser)
    }

    // For testing purposes.
    //
    pub fn default_group(&self) -> Arc<Group> {
        self.group(DEFAULT_GROUP_NAME)
    }

    // In the book, this doesn't have a specified API; it's referenced as `"group_name" from parser`.
    //
    pub fn group(&self, group_name: &str) -> Arc<Group> {
        let parsed_elements = self.groups_data.get(group_name).unwrap();

        let triangles = parsed_elements
            .iter()
            .map(|parsed_element| {
                let triangle = self.triangle_from_parsed_element(parsed_element);
                Arc::new(triangle) as Arc<dyn Shape>
            })
            .collect();

        Group::new(Matrix::identity(4), triangles)
    }

    // Convenience method for exporting the groups as tree, with the group as leaves of a new root group.
    // In the book, this is `obj_to_group()`.
    //
    pub fn export_tree(&self) -> Arc<Group> {
        let groups = self
            .groups_data
            .keys()
            .map(|group_name| self.group(group_name) as Arc<dyn Shape>)
            .collect::<Vec<_>>();

        Group::new(Matrix::identity(4), groups)
    }

    pub fn vertex(&self, i: usize) -> Tuple {
        self.vertices[i - 1]
    }

    pub fn normal(&self, i: usize) -> Tuple {
        self.normals[i - 1]
    }

    fn triangle_from_parsed_element(&self, element: &ParsedElement) -> Triangle {
        match element {
            Face(p1i, p2i, p3i) => {
                let p1 = self.vertex(*p1i);
                let p2 = self.vertex(*p2i);
                let p3 = self.vertex(*p3i);

                Triangle::new(p1, p2, p3)
            }
            FaceWithNormal((p1i, n1i), (p2i, n2i), (p3i, n3i)) => {
                let p1 = self.vertex(*p1i);
                let p2 = self.vertex(*p2i);
                let p3 = self.vertex(*p3i);
                let n1 = self.normal(*n1i);
                let n2 = self.normal(*n2i);
                let n3 = self.normal(*n3i);

                Triangle::smooth(p1, p2, p3, n1, n2, n3)
            }
            _ => {
                panic!("{:?}", element)
            }
        }
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
                .split(' ')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            for other_ps_i in all_other_ps_i.windows(2) {
                let p2i = other_ps_i[0];
                let p3i = other_ps_i[1];

                faces.push((p1i, p2i, p3i));
            }

            ParsedElement::Faces(faces)
        } else if let Some(captures) = FACES_WITH_TEXTURE_REGEX.captures(&line) {
            let p1i: usize = captures[1].parse().unwrap();
            let p2i: usize = captures[2].parse().unwrap();
            let p3i: usize = captures[3].parse().unwrap();

            ParsedElement::Face(p1i, p2i, p3i)
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
