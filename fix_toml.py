import os, shutil

with open('packages/custom_pet/behaviors.toml', 'r') as f:
    lines = f.read().splitlines()

out = []
current_state = None

inject = '''
[[states.transitions]]
condition = { type = "dragged" }
target = "dragged"
priority = 100

[[states.transitions]]
condition = { type = "falling" }
target = "fall"
priority = 90
'''

for line in lines:
    if line.startswith('[[states]]'):
        out.append(line)
    elif line.startswith('name = '):
        current_state = line.split('"')[1]
        out.append(line)
    elif line.startswith('[[states.transitions]]'):
        if current_state and current_state not in ['fall', 'dragged']:
            out.append(inject.strip())
            current_state = None
        out.append(line)
    else:
        out.append(line)

out_text = '\n'.join(out)

out_text = out_text.split('[[states]]\nname = "fall"')[0]
out_text = out_text.split('[[states]]\nname = "dragged"')[0]

out_text += '''
[[states]]
name = "dragged"
animation = "alert"

[[states.transitions]]
condition = { type = "falling" }
target = "fall"
priority = 50

[[states.transitions]]
condition = { type = "grounded" }
target = "idle"
priority = 40

[[states]]
name = "fall"
animation = "idle"

[[states.transitions]]
condition = { type = "dragged" }
target = "dragged"
priority = 100

[[states.transitions]]
condition = { type = "grounded" }
target = "idle"
priority = 50
'''

with open('packages/custom_pet/behaviors.toml', 'w') as f:
    f.write(out_text)

zip_path = 'packages/custom_pet.petpkg'
if os.path.exists(zip_path): os.remove(zip_path)
shutil.make_archive('packages/custom_pet', 'zip', 'packages/custom_pet')
os.rename('packages/custom_pet.zip', zip_path)
shutil.copy(zip_path, 'src/neko.petpkg')
