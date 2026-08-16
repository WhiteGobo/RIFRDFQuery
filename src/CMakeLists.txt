#find_library(DYNAMICLINKING_LIBRARY stdc++ REQUIRED)

#add_library(RIFRDFQuery_privateCPP
#	qq.cpp
#)

corrosion_import_crate(MANIFEST_PATH ./Cargo.toml)
corrosion_link_libraries(RIFRDFQuery
	#rocksdb
	#${DYNAMICLINKING_LIBRARY}
	#RIFRDFQuery_privateCPP
)

target_sources(RIFRDFQuery INTERFACE
        FILE_SET HEADERS
        BASE_DIRS include
	FILES
	include/RIFRDFQuery.h
)
corrosion_install(TARGETS RIFRDFQuery EXPORT RIFRDFQuerytargets)
