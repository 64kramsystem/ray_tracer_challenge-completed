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
    static ref VERTEX_REGEX: Regex = Regex::new(r"^v (-?\d(?:\.\d+)?) (-?\d(?:\.\d+)?) (-?\d(?:\.\d+)?)$").unwrap();
    static ref FACES_REGEX: Regex = Regex::new(r"^f (\d+) (\d+(?: \d+)+)$").unwrap();
}

// The book doesn't actually clarify what happens to the default group once group definitions parsing
// is introduced.
//
const DEFAULT_GROUP_NAME: &str = "default";

enum ParsedElement {
    Vertex(Tuple),
    Faces(Vec<Arc<dyn Shape>>),
    Invalid,
}

pub struct ObjParser {
    // WATCH OUT!!! DON'T ACCESS THIS DIRECTLY, WHILE PARSING!!!
    // The indexes are 1-based, which are extremely easy to mistake.
    //
    vertices: Vec<Tuple>,
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
            groups,
        };

        let current_group_name = DEFAULT_GROUP_NAME.to_string();

        for line in reader.lines() {
            let parsed_element = parser.parse_line(line?);

            match parsed_element {
                Vertex(vertex) => parser.vertices.push(vertex),
                Faces(triangles) => {
                    for triangle in triangles {
                        let group = parser.groups.entry(current_group_name.to_string());
                        group.and_modify(|group| Group::add_child(group, &triangle));
                    }
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

    pub fn vertex(&self, i: usize) -> Tuple {
        self.vertices[i - 1]
    }

    fn parse_line(&self, line: String) -> ParsedElement {
        if let Some(captures) = VERTEX_REGEX.captures(&line) {
            let x: f64 = captures[1].parse().unwrap();
            let y: f64 = captures[2].parse().unwrap();
            let z: f64 = captures[3].parse().unwrap();

            ParsedElement::Vertex(Tuple::point(x, y, z))
        } else if let Some(captures) = FACES_REGEX.captures(&line) {
            let mut faces = vec![];

            let p1i: usize = captures[1].parse().unwrap();
            let p1 = self.vertex(p1i);

            let all_other_ps_i = captures[2]
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            for other_ps_i in all_other_ps_i.windows(2) {
                let p2i = other_ps_i[0];
                let p3i = other_ps_i[1];

                // Watch out the 1-base.
                //
                let p2 = self.vertex(p2i);
                let p3 = self.vertex(p3i);

                let face: Arc<dyn Shape> = Arc::new(Triangle::new(p1, p2, p3));

                // Watch out the 1-base.
                //
                faces.push(face);
            }

            ParsedElement::Faces(faces)
        } else {
            Invalid
        }
    }
}
