#pragma once

typedef void RIFRDFQuery_Graph;

int8_t RIFRDFQuery_Graph_query_rif_data(RIFRDFQuery_Graph* data_graph, RIFRDFQuery_Graph* query_graph);
RIFRDFQuery_Graph* RIFRDFQ_Graph_from_data(const char* data, const char* media_type);
void free_RIFRDFQuery_Graph(RIFRDFQuery_Graph*);
