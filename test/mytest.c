#include <stdlib.h>
#include <stdio.h>
#include <getopt.h>
#include "RIFRDFQuery.h"

const char* datafile = NULL;
const char* queryfile = NULL;
bool expect = true;

static int parse_args(int argc, char *argv[]);
static char* load_into_memory(const char* filepath);

int main(int argc, char *argv[]){
	int err;
	char *data, *query;
	RIFRDFQuery_Graph *data_graph, *query_graph;
	err = parse_args(argc, argv);
	if (err != 0){
		exit(EXIT_FAILURE);
	}
	data = load_into_memory(datafile);
	if (data == NULL){
		fprintf(stderr, "Failed to load datafile\n");
		exit(EXIT_FAILURE);
	}
	query = load_into_memory(queryfile);
	if (query == NULL){
		fprintf(stderr, "Failed to load queryfile\n");
		exit(EXIT_FAILURE);
	}
	data_graph = RIFRDFQ_Graph_from_data(data, "ttl");
	if (data_graph == NULL){
		fprintf(stderr, "Failed to produce data graph %s\n", data);
		exit(EXIT_FAILURE);
	}
	query_graph = RIFRDFQ_Graph_from_data(query, "ttl");
	if (query_graph == NULL){
		fprintf(stderr, "Failed to produce query graph\n");
		exit(EXIT_FAILURE);
	}

	err = RIFRDFQuery_Graph_query_rif_data(data_graph, query_graph);
	free(data);
	free(query);
	free_RIFRDFQuery_Graph(data_graph);
	free_RIFRDFQuery_Graph(query_graph);
	switch(err){
		case 0:
			if (expect){
				exit(EXIT_SUCCESS);
			} else {
				fprintf(stderr, "Found queried facts but did "
						"expect failure\n");
				exit(EXIT_FAILURE);
			}
		default:
			if (expect){
				fprintf(stderr, "Failed to find expected facts\n");
				exit(EXIT_FAILURE);
			} else {
				exit(EXIT_SUCCESS);
			}
	}
}

static struct option parse_options[] = {
	{"data", required_argument, NULL, 'd'},
	{"query", required_argument, NULL, 'q'},
	{"expected-failure", no_argument, NULL, 'f'},
        {NULL, 0, NULL, 0}
};

static int parse_args(int argc, char *argv[]){
	int err = 0;
	int c = 0;
	int option_index;
	while(c != -1){
		c = getopt_long(argc, argv, "",
				parse_options, &option_index);
		switch(c){
			case -1: //end of arguments
				break;
			case 'f':
				expect = false;
				break;
			case 'd':
				datafile = optarg;
				break;
			case 'q':
				queryfile = optarg;
				break;
			default:
				fprintf(stderr, "unrecognized argument\n");
				err = 1;
				break;
		}
	}
	if (datafile == NULL) {
		fprintf(stderr, "Missing datafile\n");
		err = 1;
	}
	if (queryfile == NULL){
		fprintf(stderr, "Missing queryfile\n");
		err = 1;
	}
	return err;
}

static char* load_into_memory(const char* filepath){
        char *ret;
        long fsize;
        FILE *f = fopen(filepath, "rb");
	fprintf(stderr, "brubru %s\n", filepath);
        if (f == NULL) return NULL;
        fseek(f, 0, SEEK_END);
        fsize = ftell(f);
        rewind(f);
        //fseek(f, 0, SEEK_SET);  /* same as rewind(f); */

        ret = malloc(fsize + 1);
        fread(ret, fsize, 1, f);
        ret[fsize] = 0;
        fclose(f);
        return ret;
}
