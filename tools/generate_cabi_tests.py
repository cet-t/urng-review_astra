import json
from pathlib import Path
items=json.loads(Path('tools/cabi_inventory.json').read_text())
out=['// Exported-family coverage generated from the preserved ABI inventory.']
for item in items:
 if not item['name'].endswith('_new'): continue
 stem=item['name'][:-4]; ty=item['type']; bits=item['bits']; n=item['lanes']; feat=item['feature']
 names={i['name'] for i in items}
 fill=stem+f'_next_u{bits}s'
 if fill not in names: continue
 is_simd=n>1
 if is_simd: out.append('#[cfg(all(feature="simd",target_arch="x86_64"))]')
 guard=f'if !std::is_x86_feature_detected!("{feat}") {{ assert!({stem}_new(1).is_null()); return; }}' if feat else ''
 guard=guard.replace('!std::is_x86_feature_detected!("avx512dq")', '!std::is_x86_feature_detected!("avx512dq") || !std::is_x86_feature_detected!("avx512f")')
 method='std::mem::transmute::<_,[u32;'+str(n)+']>(r.nextuv())' if ty.startswith(('Jsf32x','Xoshiro128','Xoroshiro64Ss','SplitMix32x')) and is_simd else 'r.nextu()'
 draw=f'expected.extend({method});' if ty in ['Sfc32x4','Sfc32x8','Sfc32x16','Xoshiro256Ssx2'] else f'expected.extend(unsafe {{{method}}});' if is_simd else 'expected.push(r.nextu());'
 construct=f'unsafe {{{ty}::new(1)}}' if is_simd and ty not in ['Sfc32x4','Biski64x8','Sfc64x8','Cet64x8','Cet256x2','Xoshiro256Ssx2'] else f'{ty}::new(1)'
 extra=[]
 for suffix,word,sentinel,transform,params in [
     (f'next_f{bits}s',f'f{bits}', '-999.0',f'Word::to_f01',''),
     (f'rand_i{bits}s',f'i{bits}', '123',f'|x|x.to_randi(i{bits}::MIN,i{bits}::MAX)',f',i{bits}::MIN,i{bits}::MAX'),
     (f'rand_f{bits}s',f'f{bits}', '-999.0',f'|x|x.to_randf(-f{bits}::MAX,f{bits}::MAX)',f',-f{bits}::MAX,f{bits}::MAX')]:
  function=stem+'_'+suffix
  if function in names:
   extra.append(f'let converted:Vec<{word}>=expected.iter().copied().map({transform}).collect();')
   # Numeric exports put min/max before out/count; read the inventory to preserve order.
   args=next(i['args'] for i in items if i['name']==function)
   call=[]
   for arg in args.split(','):
    if not arg.strip(): continue
    param=arg.strip().split(':')[0]
    call.append({'ptr':'p','out':'o','count':'n','min':f'{word}::MIN' if word.startswith('i') else f'-{word}::MAX','max':f'{word}::MAX'}[param])
   extra.append(f'check_with(1,{stem}_new,{stem}_free,|p,o,n|unsafe{{{function}({",".join(call)})}},&converted,{sentinel});')
 numeric='\n    '.join(extra)
 out.append(f'''#[test]
fn {stem}_boundaries_alignment_and_partition() {{
    {guard}
    let mut r={construct};
    let mut expected=Vec::new();
    while expected.len()<1024 {{ {draw} }}
    check(1,{stem}_new,{stem}_free,{fill},&expected,u{bits}::MAX);
    {numeric}
}}''')
Path('tests/support').mkdir(exist_ok=True)
Path('tests/support/cabi_cases.rs').write_text('\n'.join(out)+'\n',encoding='utf-8',newline='\n')
