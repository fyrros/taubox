import os

tmplt = 'ebook-convert %s res/%s.mobi'
script = []
with open('e_convert.sh', 'w') as res:
	for filename in os.listdir('/home/fyrros/Downloads/kindle'):
		script.append(tmplt % (filename, filename.split('.')[0]))

	res.write('#!/bin/bash\n')
	res.write(' &&\n'.join(script))