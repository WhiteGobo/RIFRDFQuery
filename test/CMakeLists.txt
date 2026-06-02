add_executable(mytest
	mytest.c
)
target_link_libraries(mytest
	RIFRDFQuery
)

set(DATAFILE "${CMAKE_CURRENT_SOURCE_DIR}/data1.jsonld")
set(QUERYFILE "${CMAKE_CURRENT_SOURCE_DIR}/query1.jsonld")

include(tools.cmake)

set(basepath "${CMAKE_CURRENT_SOURCE_DIR}/data")
read_manifest("${basepath}/manifest.json" testarray length)

set(basename "RIFRDFQueryTest::")
math(EXPR range "${length} - 1")
foreach(x RANGE 0 ${range})
	set(extras "")
	string(JSON data GET ${testarray} ${x})
	string(JSON testsuffix GET ${data} name)
	string(JSON premise GET ${data} premise)
	string(JSON query GET ${data} query)
	string(JSON testtype GET ${data} "@type")
	if (testtype MATCHES "NegativeEntailment")
		list(APPEND extras "--expected-failure")
	endif()

	set(testname "${basename}${testsuffix}")
	add_test(NAME "${testname}" COMMAND mytest
		ARGS
		"--data" "${basepath}/${premise}"
		"--query" "${basepath}/${query}"
		${extras}
	)
	set_property(TEST "${testname}" PROPERTY LABELS "RIFRDFQuery""${testtype}")

endforeach()
