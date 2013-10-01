var searchIndex = [{ty:"mod",name:"",path:"knob",desc:"A convenient structure to store and load settings."},{ty:"struct",name:"Settings",path:"knob",desc:"The settings structure we save the options and settings in."},{ty:"structfield",name:"store",path:"knob",desc:"",parent:'15'},{ty:"structfield",name:"options",path:"knob",desc:"",parent:'15'},{ty:"method",name:"clone",path:"knob",desc:"",parent:'15'},{ty:"method",name:"new",path:"knob",desc:"Create a new Settings struct.",parent:'15'},{ty:"method",name:"set",path:"knob",desc:"Set a settings key to a value. The value will be serialized.",parent:'15'},{ty:"method",name:"set_opt",path:"knob",desc:"Set a value using an Option struct. The value will only be set if the\nSome value is given. This way, you can avoid unwrapping the result of a\nprevious operation by yourself.",parent:'15'},{ty:"method",name:"fetch",path:"knob",desc:"Fetch a setting for a key. Fails if the setting is present but could not be\nparsed.",parent:'15'},{ty:"method",name:"fetch_with",path:"knob",desc:"Fetch a setting for a key and pass it to given function. The result of the function\nwill be returned.",parent:'15'},{ty:"method",name:"opt",path:"knob",desc:"Register a commandline for laster use with load_args.",parent:'15'},{ty:"method",name:"load_os_args",path:"knob",desc:"Load the command line argument given by the OS.",parent:'15'},{ty:"method",name:"load_args",path:"knob",desc:"Load a list of command line arguments.",parent:'15'},{ty:"method",name:"usage",path:"knob",desc:"Returns the usage string for the stored OptGroups. Pass `brief`\nto have it included.",parent:'15'}];var allPaths = {'0':{type:'mod',name:'knob'},'15':{type:'struct',name:'Settings'}};