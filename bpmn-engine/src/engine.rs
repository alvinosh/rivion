use std::{
    collections::{binary_heap::Iter, HashMap},
    fmt::Display,
};

use crate::{definitions::*, graph::Graph};

pub struct BPMNEngine {
    // definitions: Definitions,
    flow: Graph<Id, FlowElement, ()>,
    current: Option<Id>,
}

impl BPMNEngine {
    pub fn new(definitions: Definitions, at: Option<Id>) -> Self {
        let mut graph: Graph<Id, FlowElement, ()> = Graph::new();
        for entry in definitions.root_elements {
            match entry {
                RootElement::Process(proccess) => {
                    for flow in proccess.flow_elements {
                        match flow {
                            FlowElement::EndEvent(event) => {
                                let id = event.id.clone().expect("ERROR: NO ID FOUND IN EVENT");
                                graph.push_vertex(id, FlowElement::EndEvent(event))
                            }
                            FlowElement::SequenceFlow(seq) => {
                                graph.push_edge(seq.source_ref, seq.target_ref, ());
                            }
                            FlowElement::StartEvent(event) => {
                                let id = event.id.clone().expect("ERROR: NO ID FOUND IN EVENT");
                                graph.push_vertex(id, FlowElement::StartEvent(event))
                            }
                            FlowElement::Task(task) => {
                                let id = task.id.clone().expect("ERROR: NO ID FOUND IN EVENT");
                                graph.push_vertex(id, FlowElement::Task(task))
                            }
                            FlowElement::EventBasedGateway(event) => {
                                let id = event.id.clone().expect("ERROR: NO ID FOUND IN EVENT");
                                graph.push_vertex(id, FlowElement::EventBasedGateway(event))
                            }
                            FlowElement::ParallelGateway(gate) => {
                                let id = gate.id.clone().expect("ERROR: NO ID FOUND IN EVENT");
                                graph.push_vertex(id, FlowElement::ParallelGateway(gate))
                            }
                            FlowElement::IntermediateCatchEvent(e) => {
                                let id = e.id.clone().expect("ERROR: NO ID FOUND IN EVENT");
                                graph.push_vertex(id, FlowElement::IntermediateCatchEvent(e))
                            }
                        }
                    }
                }
                RootElement::Message(_) => {}
                RootElement::Collaboration(_) => {}
            }
        }

        Self {
            current: None,
            flow: graph,
        }
    }

    pub fn next(&mut self, next: Id) {
        self.current = Some(next);
    }

    pub fn options(&mut self) -> Vec<&FlowElement> {
        if let Some(id) = self.current.clone() {
            let mut output = vec![];
            for val in self.flow.adjacent(&id) {
                let v = self.flow.get_vertex(val);
                output.push(v.expect("Unreachable"));
            }
            output
        } else {
            self.flow
                .iter_vertices()
                .filter_map(|x| match x.1 {
                    FlowElement::StartEvent(_) => Some(x.1),
                    _ => None,
                })
                .collect()
        }
    }
}

impl Display for BPMNEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in self.flow.iter_vertices() {
            write!(f, "{}", value.1)?;
            write!(f, "         NEIGHBORS:")?;
            for adj in self.flow.adjacent(value.0) {
                if let Some(val) = self.flow.get_vertex(adj) {
                    write!(f, "         {}", val)?;
                } else {
                    write!(f, "         Not Implemented")?
                }
            }
            write!(f, "")?;
        }
        Ok(())
    }
}
