import os

# add path to dll_directory!
def add():
    # Windows only
    if not hasattr( os, 'add_dll_directory'):
        return

    print( 'Adding dll dirs from env path' )

    paths = os.environ[ 'path' ].split( ';' )
    paths.reverse()
    for p in paths:
        if not os.path.isdir( p ):
            continue

        print( "Adding a dll dir:", p )
        os.add_dll_directory( p )


add()
