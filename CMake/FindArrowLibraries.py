import pyarrow as pa

include = pa.get_libraries()

def convert(lst):
    return "".join([char for char in str(lst) if char not in "[],"]).replace("\'", "\"")

print(convert(include))
