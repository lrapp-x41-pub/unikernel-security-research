from pwn import *
import array

canaries = []

for i in range(0, 20):
    # TODO fixed in upstream, specify old version
    p = process(['ops', 'run', 'canary'])
    p.recvuntil(b'Canary: ')
    canary = p.recvline(timeout=1.0).decode("utf-8")[:-1]
    print(canary)
    canaries.append(int(canary, 16))
    p.close()
    sleep(0.2)

print("-------------------------")
print("Canaries:")
print(canaries)
print("Sorted canaries:")
print(sorted(canaries))
print("Statistics:")
count = 0
old_value = sorted(canaries)[0]
for canary in sorted(canaries):
    if canary == old_value:
        count += 1
    else:
        print(f"{count} x {old_value}")
    count = 1
    old_value = canary
print(f"{count} x {old_value}")
