Simple python script to convert float values from the first column of a csv file to a very large integer. This was quickly put together to avoid wasting time with getting this functionality done in Rust.

# Running
By default the results will print to STDOUT, simply pipe the output to a new csv file:
`./convert_csv.py -i TIC.csv > my_new_csv_file.csv`
For debugging purposes using this script with the `-v` flag will also print the original row of the csv file.

# Issues
If you have issues running this you most likely do not have a necessary package installed. The simplest solution would be to use pip:
`pip install <package>`
See https://packaging.python.org/tutorials/installing-packages/ for more information regarding pip.