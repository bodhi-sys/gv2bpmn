include(`bpmn.m4')
start([[Start
Point]])
t(task1, [[Task with
multiline
label]])
g_xor(gate1, [[Either
A or B]])
end([[End
Point]])
to(START_NODE, task1)
g_to(task1, gate1, [[path
taken]])
to(gate1, END_NODE)
