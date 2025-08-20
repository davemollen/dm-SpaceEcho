# automatically build the wrapper only if this is the top level project
if (PROJECT_IS_TOP_LEVEL)
	# provide the CLAP_WRAPPER_OUTPUT_NAME to specify the matching plugin name.
	if((NOT CLAP_WRAPPER_OUTPUT_NAME ) OR (CLAP_WRAPPER_OUTPUT_NAME STREQUAL ""))
		set(CLAP_WRAPPER_OUTPUT_NAME "clapasvst3")
		message(WARNING "clap-wrapper: CLAP_WRAPPER_OUTPUT_NAME not set - continuing with a default name `clapasvst3`")
	endif()

	string(MAKE_C_IDENTIFIER ${CLAP_WRAPPER_OUTPUT_NAME} pluginname)

	if (APPLE)
		if (${CLAP_WRAPPER_BUILD_AUV2})
			add_library(${pluginname}_as_auv2 MODULE)
			target_add_auv2_wrapper(
					TARGET ${pluginname}_as_auv2
					OUTPUT_NAME "${CLAP_WRAPPER_OUTPUT_NAME}"
					BUNDLE_IDENTIFIER "${CLAP_WRAPPER_BUNDLE_IDENTIFIER}"
					BUNDLE_VERSION "${CLAP_WRAPPER_BUNDLE_VERSION}"

					# AUv2 is still a work in progress. For now make this an
					# explict option to the single plugin version
					INSTRUMENT_TYPE "${CLAP_WRAPPER_AUV2_INSTRUMENT_TYPE}"
					MANUFACTURER_NAME "Dave Mollen"
					MANUFACTURER_CODE "dmAd"
					SUBTYPE_CODE "${CLAP_WRAPPER_AUV2_SUBTYPE}"
			)
		endif()
	endif()
endif()
