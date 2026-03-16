# gv2bpmn

gv2bpmn is a tool for converting m4 based DSL for BPMN2.0(small subset).
It's implemented on top of GraphViz dot.
The conversion goes in several steps:

- convert m4 input via bpmn.m4 to GraphViz notation
- convert dot file to JSON via dot
- convert JSON to BPMN2.0 XML via rust tool
