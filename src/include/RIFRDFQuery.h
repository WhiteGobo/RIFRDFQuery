#pragma once
#include <stdint.h>

#ifndef RDF_TERMTYPE_DEFINED
#define RDF_TERMTYPE_DEFINED
typedef enum {
        URI = 0,
        BNODE = 1,
        TYPEDLITERAL = 2,
        LANGLITERAL = 3
} TERMTYPE;
#endif

typedef struct s_RIFRDFQuery_Graph RIFRDFQuery_Graph;

/**
 * return 0 on successfull query and 1, when the graph doesnt contain the data
 * as described in the query. On any other error, it will return a negative value
 */
int8_t RIFRDFQuery_Graph_query_rif_data(RIFRDFQuery_Graph* data_graph, RIFRDFQuery_Graph* query_graph);

RIFRDFQuery_Graph* RIFRDFQ_Graph_from_data(const char* data, const char* media_type);
void free_RIFRDFQuery_Graph(RIFRDFQuery_Graph*);
RIFRDFQuery_Graph* RIFRDFQuery_Graph_new();
int8_t RIFRDFQuery_Graph_add(
                const char* subject, uint8_t subject_type,
                const char* predicate,
                const char* object, const char* object_suffix,
                uint8_t object_type,
                const char* graphid, uint8_t graph_type,
		RIFRDFQuery_Graph* user);
