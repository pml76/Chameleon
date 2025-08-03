#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <arrow/python/pyarrow.h>

#include <iostream>

void pyarrow_test() {
    std::cout << "pyarrow_test" << std::endl;
    // arrow::py::import_pyarrow();
}


int
main(int argc, char *argv[])
{
    Py_Initialize();

    //if (const Status st = RunMain(argc, argv); !st.ok()) {
    //    std::cerr << st << std::endl;
    //    return 1;
    //}

    PyStatus status;
    PyConfig config;
    PyConfig_InitPythonConfig(&config);

    /* optional but recommended */
    status = PyConfig_SetBytesString(&config, &config.program_name, argv[0]);
    if (PyStatus_Exception(status)) {
        goto exception;
    }

    status = Py_InitializeFromConfig(&config);
    if (PyStatus_Exception(status)) {
        goto exception;
    }
    PyConfig_Clear(&config);

    PyRun_SimpleString("from time import time,ctime\n"
                       "print('Today is', ctime(time()))\n");
    if (Py_FinalizeEx() < 0) {
        exit(120);
    }
    Py_Finalize();
    return 0;

exception:
    PyConfig_Clear(&config);
    Py_ExitStatusException(status);
    Py_FinalizeEx();
}
