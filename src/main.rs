use serde::Deserialize;
use std::io::{self, Read, Write};
use quick_xml::events::{Event, BytesDecl, BytesStart, BytesEnd, BytesText};
use quick_xml::Writer;
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input Graphviz JSON file. If not provided, reads from stdin.
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct Graph {
    pub name: Option<String>,
    pub bb: Option<String>,
    pub objects: Option<Vec<Object>>,
    pub edges: Option<Vec<Edge>>,
}

#[derive(Debug, Deserialize)]
pub struct Object {
    pub _gvid: usize,
    pub name: String,
    pub label: Option<String>,
    pub shape: Option<String>,
    pub pos: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Edge {
    pub _gvid: usize,
    pub tail: usize,
    pub head: usize,
    pub label: Option<String>,
    pub pos: Option<String>,
    #[serde(rename = "lp")]
    pub label_pos: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum BpmnElement {
    StartEvent { id: String, name: String, x: f64, y: f64 },
    EndEvent { id: String, name: String, x: f64, y: f64 },
    Task { id: String, name: String, x: f64, y: f64, width: f64, height: f64 },
    ExclusiveGateway { id: String, name: String, x: f64, y: f64 },
    ParallelGateway { id: String, name: String, x: f64, y: f64 },
    InclusiveGateway { id: String, name: String, x: f64, y: f64 },
}

impl BpmnElement {
    fn id(&self) -> &str {
        match self {
            BpmnElement::StartEvent { id, .. } |
            BpmnElement::EndEvent { id, .. } |
            BpmnElement::Task { id, .. } |
            BpmnElement::ExclusiveGateway { id, .. } |
            BpmnElement::ParallelGateway { id, .. } |
            BpmnElement::InclusiveGateway { id, .. } => id,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BpmnFlow {
    pub id: String,
    pub name: String,
    pub source_ref: String,
    pub target_ref: String,
    pub waypoints: Vec<(f64, f64)>,
}

fn parse_pos(pos: &str) -> (f64, f64) {
    let parts: Vec<&str> = pos.split(',').collect();
    if parts.len() >= 2 {
        let x = parts[0].parse().unwrap_or(0.0);
        let y = parts[1].parse().unwrap_or(0.0);
        (x, y)
    } else {
        (0.0, 0.0)
    }
}

fn parse_edge_pos(pos: &str) -> Vec<(f64, f64)> {
    pos.split(' ')
        .map(|p| {
            let p = if p.starts_with('e') || p.starts_with('s') {
                if let Some(comma_pos) = p.find(',') {
                    &p[comma_pos+1..]
                } else {
                    p
                }
            } else {
                p
            };
            parse_pos(p)
        })
        .collect()
}

fn parse_bb(bb: &str) -> (f64, f64, f64, f64) {
    let parts: Vec<&str> = bb.split(',').collect();
    if parts.len() == 4 {
        let x1 = parts[0].parse().unwrap_or(0.0);
        let y1 = parts[1].parse().unwrap_or(0.0);
        let x2 = parts[2].parse().unwrap_or(0.0);
        let y2 = parts[3].parse().unwrap_or(0.0);
        (x1, y1, x2, y2)
    } else {
        (0.0, 0.0, 0.0, 0.0)
    }
}

pub fn convert_to_bpmn<W: Write>(json_str: &str, output: W) -> io::Result<()> {
    let graph: Graph = serde_json::from_str(json_str).expect("Failed to parse JSON");

    let (_, _, _, height) = graph.bb.as_deref().map(parse_bb).unwrap_or((0.0, 0.0, 0.0, 1000.0));

    let mut elements = Vec::new();
    let mut objects_map = std::collections::HashMap::new();

    if let Some(objects) = &graph.objects {
        for obj in objects {
            let id = format!("Element_{}", obj._gvid);
            let label = obj.label.clone().unwrap_or_default().trim().to_string();
            let (x, raw_y) = obj.pos.as_deref().map(parse_pos).unwrap_or((0.0, 0.0));
            let y = height - raw_y;

            let shape = obj.shape.as_deref().unwrap_or("");

            let element = if obj.name.starts_with("start__") || shape == "circle" || obj.name == "START_NODE" {
                BpmnElement::StartEvent { id: id.clone(), name: label, x, y }
            } else if obj.name.starts_with("end__") || shape == "doublecircle" || obj.name == "END_NODE" {
                BpmnElement::EndEvent { id: id.clone(), name: label, x, y }
            } else if obj.name.starts_with("g_xor__") || shape == "diamond" {
                BpmnElement::ExclusiveGateway { id: id.clone(), name: label, x: x - 25.0, y: y - 25.0 }
            } else if obj.name.starts_with("g_and__") {
                BpmnElement::ParallelGateway { id: id.clone(), name: label, x: x - 25.0, y: y - 25.0 }
            } else if obj.name.starts_with("g_or__") {
                BpmnElement::InclusiveGateway { id: id.clone(), name: label, x: x - 25.0, y: y - 25.0 }
            } else {
                let w = obj.width.as_deref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(1.38) * 72.0;
                let h = obj.height.as_deref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.5) * 72.0;
                BpmnElement::Task { id: id.clone(), name: label, x: x - w/2.0, y: y - h/2.0, width: w, height: h }
            };

            elements.push(element);
            objects_map.insert(obj._gvid, id);
        }
    }

    let mut flows = Vec::new();
    if let Some(edges) = &graph.edges {
        for edge in edges {
            let id = format!("Flow_{}", edge._gvid);
            let name = edge.label.clone().unwrap_or_default().trim().to_string();
            let source_ref = objects_map.get(&edge.tail).cloned().unwrap_or_default();
            let target_ref = objects_map.get(&edge.head).cloned().unwrap_or_default();
            let mut waypoints = edge.pos.as_deref().map(parse_edge_pos).unwrap_or_default();
            for wp in &mut waypoints {
                wp.1 = height - wp.1;
            }

            flows.push(BpmnFlow { id, name, source_ref, target_ref, waypoints });
        }
    }

    let mut writer = Writer::new_with_indent(output, b' ', 2);

    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None))).unwrap();

    let mut definitions = BytesStart::new("bpmn:definitions");
    definitions.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
    definitions.push_attribute(("xmlns:bpmn", "http://www.omg.org/spec/BPMN/20100524/MODEL"));
    definitions.push_attribute(("xmlns:bpmndi", "http://www.omg.org/spec/BPMN/20100524/DI"));
    definitions.push_attribute(("xmlns:dc", "http://www.omg.org/spec/DD/20100524/DC"));
    definitions.push_attribute(("xmlns:di", "http://www.omg.org/spec/DD/20100524/DI"));
    definitions.push_attribute(("id", "Definitions_1"));
    definitions.push_attribute(("targetNamespace", "http://bpmn.io/schema/bpmn"));
    writer.write_event(Event::Start(definitions)).unwrap();

    let mut process = BytesStart::new("bpmn:process");
    process.push_attribute(("id", "Process_1"));
    process.push_attribute(("isExecutable", "false"));
    writer.write_event(Event::Start(process)).unwrap();

    for el in &elements {
        let tag = match el {
            BpmnElement::StartEvent { .. } => "bpmn:startEvent",
            BpmnElement::EndEvent { .. } => "bpmn:endEvent",
            BpmnElement::Task { .. } => "bpmn:task",
            BpmnElement::ExclusiveGateway { .. } => "bpmn:exclusiveGateway",
            BpmnElement::ParallelGateway { .. } => "bpmn:parallelGateway",
            BpmnElement::InclusiveGateway { .. } => "bpmn:inclusiveGateway",
        };
        let mut start = BytesStart::new(tag);
        start.push_attribute(("id", el.id()));
        match el {
            BpmnElement::StartEvent { name, .. } | BpmnElement::EndEvent { name, .. } | BpmnElement::Task { name, .. } |
            BpmnElement::ExclusiveGateway { name, .. } | BpmnElement::ParallelGateway { name, .. } | BpmnElement::InclusiveGateway { name, .. } => {
                if !name.is_empty() {
                    start.push_attribute(("name", name.as_str()));
                }
            }
        }

        writer.write_event(Event::Start(start)).unwrap();
        for flow in &flows {
            if flow.source_ref == el.id() {
                let out = BytesStart::new("bpmn:outgoing");
                writer.write_event(Event::Start(out)).unwrap();
                writer.write_event(Event::Text(BytesText::new(&flow.id))).unwrap();
                writer.write_event(Event::End(BytesEnd::new("bpmn:outgoing"))).unwrap();
            }
            if flow.target_ref == el.id() {
                let inc = BytesStart::new("bpmn:incoming");
                writer.write_event(Event::Start(inc)).unwrap();
                writer.write_event(Event::Text(BytesText::new(&flow.id))).unwrap();
                writer.write_event(Event::End(BytesEnd::new("bpmn:incoming"))).unwrap();
            }
        }
        writer.write_event(Event::End(BytesEnd::new(tag))).unwrap();
    }

    for flow in &flows {
        let mut start = BytesStart::new("bpmn:sequenceFlow");
        start.push_attribute(("id", flow.id.as_str()));
        if !flow.name.is_empty() {
            start.push_attribute(("name", flow.name.as_str()));
        }
        start.push_attribute(("sourceRef", flow.source_ref.as_str()));
        start.push_attribute(("targetRef", flow.target_ref.as_str()));
        writer.write_event(Event::Empty(start)).unwrap();
    }

    writer.write_event(Event::End(BytesEnd::new("bpmn:process"))).unwrap();

    let mut diagram = BytesStart::new("bpmndi:BPMNDiagram");
    diagram.push_attribute(("id", "BPMNDiagram_1"));
    writer.write_event(Event::Start(diagram)).unwrap();

    let mut plane = BytesStart::new("bpmndi:BPMNPlane");
    plane.push_attribute(("id", "BPMNPlane_1"));
    plane.push_attribute(("bpmnElement", "Process_1"));
    writer.write_event(Event::Start(plane)).unwrap();

    for el in &elements {
        let mut shape = BytesStart::new("bpmndi:BPMNShape");
        shape.push_attribute(("id", format!("{}_di", el.id()).as_str()));
        shape.push_attribute(("bpmnElement", el.id()));
        writer.write_event(Event::Start(shape)).unwrap();

        let mut bounds = BytesStart::new("dc:Bounds");
        match el {
            BpmnElement::StartEvent { x, y, .. } | BpmnElement::EndEvent { x, y, .. } => {
                bounds.push_attribute(("x", (x - 18.0).to_string().as_str()));
                bounds.push_attribute(("y", (y - 18.0).to_string().as_str()));
                bounds.push_attribute(("width", "36"));
                bounds.push_attribute(("height", "36"));
            }
            BpmnElement::Task { x, y, width, height, .. } => {
                bounds.push_attribute(("x", x.to_string().as_str()));
                bounds.push_attribute(("y", y.to_string().as_str()));
                bounds.push_attribute(("width", width.to_string().as_str()));
                bounds.push_attribute(("height", height.to_string().as_str()));
            }
            BpmnElement::ExclusiveGateway { x, y, .. } | BpmnElement::ParallelGateway { x, y, .. } | BpmnElement::InclusiveGateway { x, y, .. } => {
                bounds.push_attribute(("x", x.to_string().as_str()));
                bounds.push_attribute(("y", y.to_string().as_str()));
                bounds.push_attribute(("width", "50"));
                bounds.push_attribute(("height", "50"));
            }
        }
        writer.write_event(Event::Empty(bounds)).unwrap();
        writer.write_event(Event::End(BytesEnd::new("bpmndi:BPMNShape"))).unwrap();
    }

    for flow in &flows {
        let mut edge = BytesStart::new("bpmndi:BPMNEdge");
        edge.push_attribute(("id", format!("{}_di", flow.id).as_str()));
        edge.push_attribute(("bpmnElement", flow.id.as_str()));
        writer.write_event(Event::Start(edge)).unwrap();

        for (x, y) in &flow.waypoints {
            let mut wp = BytesStart::new("di:waypoint");
            wp.push_attribute(("x", x.to_string().as_str()));
            wp.push_attribute(("y", y.to_string().as_str()));
            writer.write_event(Event::Empty(wp)).unwrap();
        }
        writer.write_event(Event::End(BytesEnd::new("bpmndi:BPMNEdge"))).unwrap();
    }

    writer.write_event(Event::End(BytesEnd::new("bpmndi:BPMNPlane"))).unwrap();
    writer.write_event(Event::End(BytesEnd::new("bpmndi:BPMNDiagram"))).unwrap();

    writer.write_event(Event::End(BytesEnd::new("bpmn:definitions"))).unwrap();

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut buffer = String::new();

    match args.input {
        Some(path) => {
            let mut file = File::open(path)?;
            file.read_to_string(&mut buffer)?;
        }
        None => {
            io::stdin().read_to_string(&mut buffer)?;
        }
    }

    convert_to_bpmn(&buffer, io::stdout())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pos() {
        assert_eq!(parse_pos("10,20"), (10.0, 20.0));
        assert_eq!(parse_pos("10.5,20.7"), (10.5, 20.7));
        assert_eq!(parse_pos("invalid"), (0.0, 0.0));
    }

    #[test]
    fn test_parse_edge_pos() {
        let pos = "s,10,10 20,20 30,30 e,40,40";
        let expected = vec![(10.0, 10.0), (20.0, 20.0), (30.0, 30.0), (40.0, 40.0)];
        assert_eq!(parse_edge_pos(pos), expected);
    }

    #[test]
    fn test_parse_bb() {
        assert_eq!(parse_bb("0,0,100,200"), (0.0, 0.0, 100.0, 200.0));
        assert_eq!(parse_bb("invalid"), (0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_convert_to_bpmn_basic() {
        let json = r#"{
            "bb": "0,0,100,100",
            "objects": [
                {
                    "_gvid": 0,
                    "name": "start__node",
                    "label": "Start",
                    "pos": "50,80",
                    "shape": "circle"
                },
                {
                    "_gvid": 1,
                    "name": "t__node",
                    "label": "Task",
                    "pos": "50,40",
                    "width": "1",
                    "height": "0.5"
                }
            ],
            "edges": [
                {
                    "_gvid": 0,
                    "tail": 0,
                    "head": 1,
                    "pos": "50,70 50,50"
                }
            ]
        }"#;
        let mut output = Vec::new();
        convert_to_bpmn(json, &mut output).unwrap();
        let xml = String::from_utf8(output).unwrap();

        assert!(xml.contains("bpmn:startEvent"));
        assert!(xml.contains("bpmn:task"));
        assert!(xml.contains("bpmn:sequenceFlow"));
        assert!(xml.contains("bpmndi:BPMNDiagram"));
    }

    #[test]
    fn test_integration_full_mapping() {
        let json = r#"{
            "bb": "0,0,100,100",
            "objects": [
                { "_gvid": 0, "name": "START_NODE", "label": "Start", "pos": "50,90", "shape": "circle" },
                { "_gvid": 1, "name": "t__task1", "label": "Task 1", "pos": "50,70", "width": "1", "height": "0.5" },
                { "_gvid": 2, "name": "g_xor__split", "label": "Split", "pos": "50,50", "shape": "diamond" },
                { "_gvid": 3, "name": "end__finish", "label": "End", "pos": "50,10", "shape": "doublecircle" }
            ],
            "edges": [
                { "_gvid": 0, "tail": 0, "head": 1, "pos": "50,90 50,70" },
                { "_gvid": 1, "tail": 1, "head": 2, "pos": "50,70 50,50" },
                { "_gvid": 2, "tail": 2, "head": 3, "pos": "50,50 50,10" }
            ]
        }"#;
        let mut output = Vec::new();
        convert_to_bpmn(json, &mut output).unwrap();
        let xml = String::from_utf8(output).unwrap();

        assert!(xml.contains(r#"id="Element_0""#));
        assert!(xml.contains(r#"name="Start""#));
        assert!(xml.contains("<bpmn:startEvent"));
        assert!(xml.contains("<bpmn:task"));
        assert!(xml.contains("<bpmn:exclusiveGateway"));
        assert!(xml.contains("<bpmn:endEvent"));
        assert!(xml.contains(r#"sourceRef="Element_0" targetRef="Element_1""#));
        assert!(xml.contains(r#"sourceRef="Element_1" targetRef="Element_2""#));
        assert!(xml.contains(r#"sourceRef="Element_2" targetRef="Element_3""#));
    }

    #[test]
    fn test_gateway_types() {
        let json = r#"{
            "bb": "0,0,100,100",
            "objects": [
                { "_gvid": 0, "name": "g_and__parallel", "label": "AND", "pos": "50,50" },
                { "_gvid": 1, "name": "g_or__inclusive", "label": "OR", "pos": "50,20" }
            ]
        }"#;
        let mut output = Vec::new();
        convert_to_bpmn(json, &mut output).unwrap();
        let xml = String::from_utf8(output).unwrap();

        assert!(xml.contains("<bpmn:parallelGateway"));
        assert!(xml.contains("<bpmn:inclusiveGateway"));
    }

    #[test]
    fn test_coordinate_flip() {
        // Height is 200. Pos 50,50 should become 50, 150 (200-50).
        let json = r#"{
            "bb": "0,0,100,200",
            "objects": [
                { "_gvid": 0, "name": "start__node", "pos": "50,50", "shape": "circle" }
            ]
        }"#;
        let mut output = Vec::new();
        convert_to_bpmn(json, &mut output).unwrap();
        let xml = String::from_utf8(output).unwrap();

        // StartEvent is centered at pos, so Bounds y should be y - 18.
        // raw y = 50 -> flipped y = 200 - 50 = 150.
        // Bounds y = 150 - 18 = 132.
        assert!(xml.contains(r#"y="132""#));
    }
}
