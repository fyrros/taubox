import os

root = '/home/fyrros/workspace/projects/kingdom_game2/PythonScripts/LogicServer/runing/service/core28'
root2 = '/home/fyrros/workspace/projects/kingdom_game2/PythonScripts/LogicServer/runing/service/core28/mobs'

for r, subdirs, files in os.walk(root):
	for f in files:
		path = '%s/%s' % (r,f)
		with open(path, 'r+') as script:
			data = script.read()
			script.seek(0)
			script.write(data.replace('core23', 'core28'))
			print path