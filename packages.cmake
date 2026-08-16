include(FetchContent)

FetchContent_Declare(
	Corrosion
	GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
	GIT_TAG v0.6
	FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(Corrosion)


#Needed for testing:
FetchContent_Declare(
	CInterfaceRIF
	GIT_REPOSITORY https://github.com/WhiteGobo/CInterfaceRIF.git
	FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(CInterfaceRIF)
