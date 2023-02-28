use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strong_xml::{XmlRead, XmlWrite};

pub type Id = String;
pub type URI = String;

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "bpmn:rootElement")]
pub enum RootElement {
    #[xml(tag = "bpmn:collaboration")]
    Collaboration(Collaboration),
    #[xml(tag = "bpmn:process")]
    Process(Process),
    #[xml(tag = "bpmn:message")]
    Message(Message),
}

#[derive(Debug, XmlWrite, XmlRead)]
#[xml(tag = "bpmn:definitions")]
pub struct Definitions {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(
        child = "bpmn:collaboration",
        child = "bpmn:message",
        child = "bpmn:process"
    )]
    pub root_elements: Vec<RootElement>,
}

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "bpmn:collaboration")]
pub struct Collaboration {
    #[xml(attr = "id")]
    id: Option<Id>,
    #[xml(attr = "name")]
    name: Option<String>,
    #[xml(attr = "is_closed")]
    is_closed: Option<bool>,
    #[xml(child = "bpmn:participant")]
    participants: Vec<Participant>,
    #[xml(child = "bpmn:messageFlow")]
    message_flows: Vec<MessageFlow>,
}

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "bpmn:participant")]
pub struct Participant {
    #[xml(attr = "id")]
    id: Option<Id>,
    #[xml(attr = "name")]
    name: Option<String>,
}

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "bpmn:messageFlow")]
pub struct MessageFlow {
    #[xml(attr = "id")]
    id: Option<Id>,
    #[xml(attr = "name")]
    name: Option<String>,
    #[xml(attr = "sourceRef")]
    source_ref: String,
    #[xml(attr = "targetRef")]
    target_ref: String,
    #[xml(attr = "messageRef")]
    message_ref: Option<String>,
}

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "bpmn:process")]
pub struct Process {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(child = "bpmn:laneSet")]
    pub lane_sets: Vec<LaneSet>,
    #[xml(
        child = "bpmn:endEvent",
        child = "bpmn:startEvent",
        child = "bpmn:task",
        child = "bpmn:sequenceFlow",
        child = "bpmn:eventBasedGateway",
        child = "bpmn:parallelGateway",
        child = "bpmn:intermediateCatchEvent"
    )]
    pub flow_elements: Vec<FlowElement>,
}

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "bpmn:laneSet")]
pub struct LaneSet {
    #[xml(attr = "id")]
    id: Option<Id>,
    #[xml(attr = "name")]
    name: Option<String>,
    #[xml(child = "bpmn:lane")]
    lanes: Vec<Lane>,
}

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "bpmn:lane")]
pub struct Lane {
    #[xml(attr = "id")]
    id: Option<Id>,
    #[xml(attr = "name")]
    name: Option<String>,
}

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "bpmn:message")]
pub struct Message {
    #[xml(attr = "id")]
    id: Option<Id>,
    #[xml(attr = "name")]
    name: Option<String>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[xml(tag = "bpmn:flowElement")]
pub enum FlowElement {
    #[xml(tag = "bpmn:endEvent")]
    EndEvent(EndEvent),
    #[xml(tag = "bpmn:sequenceFlow")]
    SequenceFlow(SequenceFlow),
    #[xml(tag = "bpmn:startEvent")]
    StartEvent(StartEvent),
    #[xml(tag = "bpmn:task")]
    Task(Task),
    // AdHocSubProcess(AdHocSubProcess),
    // BoundaryEvent(BoundaryEvent),
    // BusinessRuleTask(BusinessRuleTask),
    // CallActivity(CallActivity),
    // CallChoreography(CallChoreography),
    // ChoreographyTask(ChoreographyTask),
    // ComplexGateway(ComplexGateway),
    // DataObject(DataObject),
    // DataObjectReference(DataObjectReference),
    // DataStoreReference(DataStoreReference),
    // Event(Event),
    #[xml(tag = "bpmn:eventBasedGateway")]
    EventBasedGateway(EventBasedGateway),
    // ExclusiveGateway(ExclusiveGateway),
    // ImplicitThrowEvent(ImplicitThrowEvent),
    // InclusiveGateway(InclusiveGateway),
    #[xml(tag = "bpmn:intermediateCatchEvent")]
    IntermediateCatchEvent(IntermediateCatchEvent),
    // IntermediateThrowEvent(IntermediateThrowEvent),
    // ManualTask(ManualTask),
    #[xml(tag = "bpmn:parallelGateway")]
    ParallelGateway(ParallelGateway),
    // ReceiveTask(ReceiveTask),
    // ScriptTask(ScriptTask),
    // SendTask(SendTask),
    // ServiceTask(ServiceTask),
    // SubChoreography(SubChoreography),
    // SubProcess(SubProcess)
    // Transaction(Transaction),
    // UserTask(UserTask),
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[xml(tag = "bpmn:startEvent")]
pub struct StartEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    pub outgoings: Vec<String>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[xml(tag = "bpmn:endEvent")]
pub struct EndEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    pub outgoings: Vec<String>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[xml(tag = "bpmn:task")]
pub struct Task {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    pub outgoings: Vec<String>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[xml(tag = "bpmn:sequenceFlow")]
pub struct SequenceFlow {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(attr = "sourceRef")]
    pub source_ref: String,
    #[xml(attr = "targetRef")]
    pub target_ref: String,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[xml(tag = "bpmn:eventBasedGateway")]
pub struct EventBasedGateway {
    #[xml(attr = "id")]
    pub id: Option<Id>,
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(flatten_text = "bpmn:incoming")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    pub outgoings: Vec<String>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[xml(tag = "bpmn:parallelGateway")]
pub struct ParallelGateway {
    #[xml(attr = "id")]
    pub id: Option<Id>,

    #[xml(attr = "name")]
    pub name: Option<String>,

    #[xml(flatten_text = "bpmn:incoming")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    pub outgoings: Vec<String>,
}

#[derive(Debug, XmlRead, XmlWrite, Clone)]
#[xml(tag = "bpmn:intermediateCatchEvent")]
pub struct IntermediateCatchEvent {
    #[xml(attr = "id")]
    pub id: Option<Id>,

    #[xml(attr = "name")]
    pub name: Option<String>,

    #[xml(flatten_text = "bpmn:incoming")]
    pub incomings: Vec<String>,
    #[xml(flatten_text = "bpmn:outgoing")]
    pub outgoings: Vec<String>,
}

impl PartialEq for FlowElement {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::EndEvent(l0), Self::EndEvent(r0)) => l0.id == r0.id,
            (Self::SequenceFlow(l0), Self::SequenceFlow(r0)) => l0.id == r0.id,
            (Self::StartEvent(l0), Self::StartEvent(r0)) => l0.id == r0.id,
            (Self::Task(l0), Self::Task(r0)) => l0.id == r0.id,
            (Self::EventBasedGateway(l0), Self::EventBasedGateway(r0)) => l0.id == r0.id,
            (Self::IntermediateCatchEvent(l0), Self::IntermediateCatchEvent(r0)) => l0.id == r0.id,
            (Self::ParallelGateway(l0), Self::ParallelGateway(r0)) => l0.id == r0.id,
            _ => false,
        }
    }
}

impl FlowElement {
    pub fn get_id_name(&self) -> (Option<String>, Option<String>) {
        match self {
            FlowElement::EndEvent(e) => (e.id.clone(), e.name.clone()),
            FlowElement::SequenceFlow(e) => (e.id.clone(), e.name.clone()),
            FlowElement::StartEvent(e) => (e.id.clone(), e.name.clone()),
            FlowElement::Task(e) => (e.id.clone(), e.name.clone()),
            FlowElement::EventBasedGateway(e) => (e.id.clone(), e.name.clone()),
            FlowElement::IntermediateCatchEvent(e) => (e.id.clone(), e.name.clone()),
            FlowElement::ParallelGateway(e) => (e.id.clone(), e.name.clone()),
        }
    }
}

impl Display for FlowElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowElement::EndEvent(e) => {
                write!(f, "End      {:?}", e.name)
            }
            FlowElement::SequenceFlow(e) => {
                write!(f, "Flow     {:?} -> {:?}", e.source_ref, e.target_ref)
            }
            FlowElement::StartEvent(e) => {
                write!(f, "Start    {:?}", e.name)
            }
            FlowElement::Task(e) => {
                write!(f, "Task     {:?}", e.name)
            }
            FlowElement::EventBasedGateway(e) => {
                write!(f, "G.Event  {:?}", e.name)
            }
            FlowElement::ParallelGateway(e) => {
                write!(f, "P.Event  {:?}", e.name)
            }
            FlowElement::IntermediateCatchEvent(e) => {
                write!(f, "IC.Event {:?}", e.name)
            }
        }
    }
}
