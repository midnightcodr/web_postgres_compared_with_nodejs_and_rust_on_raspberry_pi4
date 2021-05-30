    {
    "apps" : [{
      "name"      : "api",
      "script"    : "main.js",// name of the startup file
      "instances" : 4,          // number of workers you want to run
      "exec_mode" : "cluster",  // to turn on cluster mode; defaults to 'fork' mode 
    }]
    }
