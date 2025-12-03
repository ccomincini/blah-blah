! macro NSIS_HOOK_POSTINSTALL
	CopyFiles "$INSTDIR\binaries\*.dll" "$INSTDIR"
!macroend
