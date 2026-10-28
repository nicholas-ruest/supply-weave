import{existsSync,readdirSync,readFileSync}from'node:fs';
const crates=readdirSync('crates').filter(x=>existsSync('crates/'+x+'/Cargo.toml'));
const adr=readdirSync('docs/adr').filter(x=>/^[0-9]{4}-/.test(x));
const ddd=readdirSync('docs/ddd').filter(x=>/^[0-9]{2}-/.test(x));
const need=['docs/specification.md','docs/architecture.md','docs/research.md','docs/implementation-contract.md','docs/traceability.md','docs/core-memory-guidance-map.md','docs/prototype-acceptance.md'];
for(const f of need)if(!existsSync(f))throw new Error('missing '+f);
if(crates.length<20||adr.length<25||ddd.length<12)throw new Error(JSON.stringify({crates:crates.length,adr:adr.length,ddd:ddd.length}));
const architecture=readFileSync('docs/architecture.md','utf8');
for(const c of crates){const p='crates/'+c+'/src/'+(existsSync('crates/'+c+'/src/lib.rs')?'lib.rs':'main.rs');const source=readFileSync(p,'utf8');if(!architecture.includes(c)||!source.includes('#[cfg(test)]')||(!source.includes('pub ')&&!p.endsWith('main.rs')))throw new Error('missing API/test/traceability for '+c)}
console.log(JSON.stringify({status:'STRUCTURE_PASS',planned_and_implemented_crates:crates.length,adr:adr.length,ddd:ddd.length}));
