
function(read_manifest manifest_path var_testarray_json var_testarray_length)

	file(READ "${manifest_path}" MANIFEST_STRING)
	string(JSON tmp
		GET ${MANIFEST_STRING} sequence
	)
	set(${var_testarray_json} ${tmp})
	string(JSON ${var_testarray_length}
		LENGTH ${${var_testarray_json}}
	)

	return(PROPAGATE
		${var_testarray_json}
		${var_testarray_length})
endfunction()
