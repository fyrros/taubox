import os

folder = 'ALL'

for root, _, files in os.walk(folder):
    result = []
    for filename in files:
        if filename.endswith('.csv') and not filename.startswith('cards'):
            with open(os.path.join(root, filename)) as csv_source:
                for line in csv_source.readlines():
                    if line[0].isdigit():
                        result.append(line.split(',')[0])
    if result:
        with open(os.path.join(root, f'{root}.csv'), 'w') as csv_result:
            csv_result.write('\n'.join(result))
            print(root, 'done')

x = input('END')
