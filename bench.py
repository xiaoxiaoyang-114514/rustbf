import subprocess, time
subprocess.run(['cargo', 'build', '--release'])
subprocess.run(['cp', r'./target/release/rustbf.exe', r'./bench/LuoTianyi-arm64'])
txb = time.time()
subprocess.run(['./bench/xiaoxiaoyang-114514/rustbf.exe', r'./bench/code/easy-opt.bf'], encoding="utf-8")
txe = time.time()
tlb = time.time()
subprocess.run([r'./bench/LuoTianyi-arm64/rustbf.exe', r'./bench/code/easy-opt.bf'], encoding="utf-8")
tle = time.time()
tx = txe - txb
tl = tle - tlb
print(f'xiaoxiaoyang-114514: {tx:.3f}')
print(f'LuoTianyi-arm64: {tl:.3f}')