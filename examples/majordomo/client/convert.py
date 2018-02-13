#!/usr/bin/env python
# By: Ryan Joyce <ryan.joyce@wsu.edu>

try:
	import argparse
	import csv
	import sys
	import re
except:
	print('Error importing necessary modules, check if they are installed.')
	sys.exit(1)

def main(args):
	with open(args.infile) as file:
		for row in file:
			split = re.split(',', row)
			maths = int(float(split[0]) * 10000000000)
			if(args.verbose):
				print(row, end="")
			print('{},{}'.format(maths, split[1]), end='')
	sys.exit(0)

if __name__ == '__main__':
	parser = argparse.ArgumentParser(description='Convert first row of a CSV file from a float value to a 64-bit unsigned integer to STDOUT')
	parser.add_argument('-i', '--infile', help='Input CSV file to convert')
	parser.add_argument('-v', '--verbose', action='store_true', help='Adds printing the original csv row during STDOUT')
	args = parser.parse_args()
	if not args.infile:
		parser.print_usage()
		print('[!] Error: No input CSV file specified!')
		sys.exit(1)
	main(args)