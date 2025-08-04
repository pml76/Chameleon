#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <arrow/api.h>
#include <arrow/compute/api.h>
#include <arrow/csv/api.h>
#include <arrow/csv/writer.h>
#include <arrow/io/api.h>
#include <arrow/result.h>
#include <arrow/status.h>

#include <iostream>
#include <vector>

#include <arrow/python/pyarrow.h>


void pyarrow_test() {
    std::cout << "pyarrow_test" << std::endl;
    arrow::py::import_pyarrow();
}


arrow::Status RunMain(int argc, char** argv) {

    return arrow::Status::OK();

    ARROW_RETURN_NOT_OK(arrow::compute::Initialize());




    // Make Arrays
  arrow::NumericBuilder<arrow::Int64Type> int64_builder;
  arrow::BooleanBuilder boolean_builder;

  // Make place for 8 values in total
  ARROW_RETURN_NOT_OK(int64_builder.Resize(8));
  ARROW_RETURN_NOT_OK(boolean_builder.Resize(8));




  // Bulk append the given values
  std::vector<int64_t> int64_values = {1, 2, 3, 4, 5, 6, 7, 8};
  ARROW_RETURN_NOT_OK(int64_builder.AppendValues(int64_values));
  std::shared_ptr<arrow::Array> array_a;
  ARROW_RETURN_NOT_OK(int64_builder.Finish(&array_a));
  int64_builder.Reset();
  int64_values = {2, 5, 1, 3, 6, 2, 7, 4};
  std::shared_ptr<arrow::Array> array_b;
  ARROW_RETURN_NOT_OK(int64_builder.AppendValues(int64_values));
  ARROW_RETURN_NOT_OK(int64_builder.Finish(&array_b));

  // Cast the arrays to their actual types
  auto int64_array_a = std::static_pointer_cast<arrow::Int64Array>(array_a);
  auto int64_array_b = std::static_pointer_cast<arrow::Int64Array>(array_b);
  // Explicit comparison of values using a loop
  for (int64_t i = 0; i < 8; i++) {
    if ((!int64_array_a->IsNull(i)) && (!int64_array_b->IsNull(i))) {
      bool comparison_result = int64_array_a->Value(i) > int64_array_b->Value(i);
      boolean_builder.UnsafeAppend(comparison_result);
    } else {
      boolean_builder.UnsafeAppendNull();
    }
  }
  std::shared_ptr<arrow::Array> array_a_gt_b_self;
  ARROW_RETURN_NOT_OK(boolean_builder.Finish(&array_a_gt_b_self));
  std::cout << "Array explicitly compared" << std::endl;

  // Explicit comparison of values using a compute function
  ARROW_ASSIGN_OR_RAISE(arrow::Datum compared_datum,
                        arrow::compute::CallFunction("greater", {array_a, array_b}));
  auto array_a_gt_b_compute = compared_datum.make_array();
  std::cout << "Arrays compared using a compute function" << std::endl;

  // Create a table for the output
  auto schema =
      arrow::schema({arrow::field("a", arrow::int64()), arrow::field("b", arrow::int64()),
                     arrow::field("a>b? (self written)", arrow::boolean()),
                     arrow::field("a>b? (arrow)", arrow::boolean())});
  std::shared_ptr<arrow::Table> my_table = arrow::Table::Make(
      schema, {array_a, array_b, array_a_gt_b_self, array_a_gt_b_compute});

  std::cout << "Table created" << std::endl;

  // Write table to CSV file
  auto csv_filename = "compute_and_write_output.csv";
  ARROW_ASSIGN_OR_RAISE(auto outstream, arrow::io::FileOutputStream::Open(csv_filename));

  std::cout << "Writing CSV file" << std::endl;
  ARROW_RETURN_NOT_OK(arrow::csv::WriteCSV(
      *my_table, arrow::csv::WriteOptions::Defaults(), outstream.get()));

  return arrow::Status::OK();
}




int main(int argc, char **argv)
{


//    if (const arrow::Status status = RunMain(argc, argv); !status.ok()) {
//        std::cerr << status.ToString() << std::endl;
//        return EXIT_FAILURE;
//    }


    arrow::Status ss;
    arrow::GlobalOptions options;
    PyConfig config;
    PyConfig_InitPythonConfig(&config);
    // config.isolated = 1;

    /* Decode command line arguments.
       Implicitly preinitialize Python (in isolated mode). */
    PyStatus status = PyConfig_SetBytesArgv(&config, argc, argv);
    if (PyStatus_Exception(status)) {
        goto exception;
    }

    status = Py_InitializeFromConfig(&config);
    if (PyStatus_Exception(status)) {
        goto exception;
    }
    PyConfig_Clear(&config);



    options.timezone_db_path = R"(C:\Users\U439644\Projects\Chameleon\extern\tzdata)";
    ss = arrow::Initialize(options);


    return Py_RunMain();

    exception:
        PyConfig_Clear(&config);
    if (PyStatus_IsExit(status)) {
        return status.exitcode;
    }
    /* Display the error message and exit the process with
       non-zero exit code */
    Py_ExitStatusException(status);
}
