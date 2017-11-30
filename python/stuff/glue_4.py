import os

folder = 'ALL'


def check_filename(name, result_name):
    return name.endswith('.csv') and not (name.startswith('cards') or name.startswith(result_name))


try:
    for root, _, files in os.walk(folder):
        result = []
        result_name = f'{os.path.basename(root)}.csv'
        for filename in files:
            if check_filename(filename, result_name):
                with open(os.path.join(root, filename)) as csv_source:
                    for line in csv_source.readlines():
                        if line[0].isdigit():
                            result.append(line.split(',')[0])
        if result:
            with open(os.path.join(root, result_name), 'w') as csv_result:
                csv_result.write('\n'.join(result))
                print(root, 'done')
except Exception as e:
    print(e)

x = input('END')
