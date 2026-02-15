/**
 * bitcoin.js — Minimal pure-JS Bitcoin address utilities for educational use.
 * Implements SHA-256, RIPEMD-160, Base58Check, and Bech32 from scratch.
 * NO external dependencies.
 *
 * ⚠️  FOR EDUCATIONAL PURPOSES ONLY – never use for real funds.
 */
const Bitcoin = (() => {
    // ────────────── SHA-256 ──────────────
    const SHA256_K = new Uint32Array([
        0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
        0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
        0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
        0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
        0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
        0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
        0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
        0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
    ]);
    function sha256(data) {
        if (typeof data === 'string') data = new TextEncoder().encode(data);
        if (data instanceof ArrayBuffer) data = new Uint8Array(data);
        const len = data.length;
        const bitLen = len * 8;
        const padLen = ((56 - (len + 1) % 64) + 64) % 64;
        const msg = new Uint8Array(len + 1 + padLen + 8);
        msg.set(data); msg[len] = 0x80;
        const dv = new DataView(msg.buffer);
        dv.setUint32(msg.length - 4, bitLen, false);
        let h0=0x6a09e667,h1=0xbb67ae85,h2=0x3c6ef372,h3=0xa54ff53a,h4=0x510e527f,h5=0x9b05688c,h6=0x1f83d9ab,h7=0x5be0cd19;
        const w = new Uint32Array(64);
        for (let off = 0; off < msg.length; off += 64) {
            for (let i=0;i<16;i++) w[i] = dv.getUint32(off + i*4, false);
            for (let i=16;i<64;i++) {
                const s0 = (ror(w[i-15],7))^(ror(w[i-15],18))^(w[i-15]>>>3);
                const s1 = (ror(w[i-2],17))^(ror(w[i-2],19))^(w[i-2]>>>10);
                w[i] = (w[i-16]+s0+w[i-7]+s1)|0;
            }
            let a=h0,b=h1,c=h2,d=h3,e=h4,f=h5,g=h6,h=h7;
            for (let i=0;i<64;i++) {
                const S1=(ror(e,6))^(ror(e,11))^(ror(e,25));
                const ch=(e&f)^((~e)&g);
                const t1=(h+S1+ch+SHA256_K[i]+w[i])|0;
                const S0=(ror(a,2))^(ror(a,13))^(ror(a,22));
                const maj=(a&b)^(a&c)^(b&c);
                const t2=(S0+maj)|0;
                h=g;g=f;f=e;e=(d+t1)|0;d=c;c=b;b=a;a=(t1+t2)|0;
            }
            h0=(h0+a)|0;h1=(h1+b)|0;h2=(h2+c)|0;h3=(h3+d)|0;h4=(h4+e)|0;h5=(h5+f)|0;h6=(h6+g)|0;h7=(h7+h)|0;
        }
        const out = new Uint8Array(32);
        const odv = new DataView(out.buffer);
        [h0,h1,h2,h3,h4,h5,h6,h7].forEach((v,i)=>odv.setUint32(i*4,v,false));
        return out;
    }
    function ror(v,n){return(v>>>n)|(v<<(32-n));}

    // ────────────── RIPEMD-160 ──────────────
    function ripemd160(data) {
        if (typeof data === 'string') data = new TextEncoder().encode(data);
        const len = data.length;
        const bitLen = len * 8;
        const padLen = ((56 - (len + 1) % 64) + 64) % 64;
        const msg = new Uint8Array(len + 1 + padLen + 8);
        msg.set(data); msg[len] = 0x80;
        const dv = new DataView(msg.buffer);
        dv.setUint32(msg.length - 8, bitLen, true);
        let h0=0x67452301,h1=0xefcdab89,h2=0x98badcfe,h3=0x10325476,h4=0xc3d2e1f0;
        const rl=[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,7,4,13,1,10,6,15,3,12,0,9,5,2,14,11,8,3,10,14,4,9,15,8,1,2,7,0,6,13,11,5,12,1,9,11,10,0,8,12,4,13,3,7,15,14,5,6,2,4,0,5,9,7,12,2,10,14,1,3,8,11,6,15,13];
        const rr=[5,14,7,0,9,2,11,4,13,6,15,8,1,10,3,12,6,11,3,7,0,13,5,10,14,15,8,12,4,9,1,2,15,5,1,3,7,14,6,9,11,8,12,2,10,0,4,13,8,6,4,1,3,11,15,0,5,12,2,13,9,7,10,14,12,15,10,4,1,5,8,7,6,2,13,14,0,3,9,11];
        const sl=[11,14,15,12,5,8,7,9,11,13,14,15,6,7,9,8,7,6,8,13,11,9,7,15,7,12,15,9,11,7,13,12,11,13,6,7,14,9,13,15,14,8,13,6,5,12,7,5,11,12,14,15,14,15,9,8,9,14,5,6,8,6,5,12,9,15,5,11,6,8,13,12,5,12,13,14,11,8,5,6];
        const sr=[8,9,9,11,13,15,15,5,7,7,8,11,14,14,12,6,9,13,15,7,12,8,9,11,7,7,12,7,6,15,13,11,9,7,15,11,8,6,6,14,12,13,5,14,13,13,7,5,15,5,8,11,14,14,6,14,6,9,12,9,12,5,15,8,8,5,12,9,12,5,14,6,8,13,6,5,15,13,11,11];
        const kl=[0,0x5a827999,0x6ed9eba1,0x8f1bbcdc,0xa953fd4e];
        const kr=[0x50a28be6,0x5c4dd124,0x6d703ef3,0x7a6d76e9,0];
        function f(j,x,y,z){if(j<16)return x^y^z;if(j<32)return(x&y)|((~x)&z);if(j<48)return(x|(~y))^z;if(j<64)return(x&z)|(y&(~z));return x^(y|(~z));}
        function rol(v,n){return((v<<n)|(v>>>(32-n)))>>>0;}
        for(let off=0;off<msg.length;off+=64){
            const w=[];for(let i=0;i<16;i++)w[i]=dv.getUint32(off+i*4,true);
            let al=h0,bl=h1,cl=h2,dl=h3,el=h4,ar=h0,br=h1,cr=h2,dr=h3,er=h4;
            for(let j=0;j<80;j++){
                let t=(al+f(j,bl,cl,dl)+w[rl[j]]+kl[j/16|0])>>>0;
                t=(rol(t,sl[j])+el)>>>0;
                al=el;el=dl;dl=rol(cl,10);cl=bl;bl=t;
                t=(ar+f(79-j,br,cr,dr)+w[rr[j]]+kr[j/16|0])>>>0;
                t=(rol(t,sr[j])+er)>>>0;
                ar=er;er=dr;dr=rol(cr,10);cr=br;br=t;
            }
            const t=(h1+cl+dr)>>>0;
            h1=(h2+dl+er)>>>0;h2=(h3+el+ar)>>>0;h3=(h4+al+br)>>>0;h4=(h0+bl+cr)>>>0;h0=t;
        }
        const out=new Uint8Array(20);
        const odv=new DataView(out.buffer);
        [h0,h1,h2,h3,h4].forEach((v,i)=>odv.setUint32(i*4,v,true));
        return out;
    }

    // ────────────── HASH160 ──────────────
    function hash160(data) { return ripemd160(sha256(data)); }

    // ────────────── HEX ──────────────
    function toHex(arr) { return Array.from(arr).map(b=>b.toString(16).padStart(2,'0')).join(''); }
    function fromHex(h) { const a=[];for(let i=0;i<h.length;i+=2)a.push(parseInt(h.substr(i,2),16));return new Uint8Array(a); }

    // ────────────── BASE58CHECK ──────────────
    const B58='123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
    function base58encode(payload) {
        let num = BigInt('0x' + toHex(payload));
        let encoded = '';
        while (num > 0n) { encoded = B58[Number(num % 58n)] + encoded; num = num / 58n; }
        for (const b of payload) { if (b === 0) encoded = '1' + encoded; else break; }
        return encoded;
    }
    function base58decode(str) {
        let num = 0n;
        for (const c of str) {
            const idx = B58.indexOf(c);
            if (idx < 0) throw new Error(`Invalid Base58 character: ${c}`);
            num = num * 58n + BigInt(idx);
        }
        let hex = num.toString(16);
        if (hex.length % 2) hex = '0' + hex;
        const bytes = fromHex(hex);
        let leading = 0;
        for (const c of str) { if (c === '1') leading++; else break; }
        const result = new Uint8Array(leading + bytes.length);
        result.set(bytes, leading);
        return result;
    }
    function base58checkEncode(version, payload) {
        const data = new Uint8Array(1 + payload.length);
        data[0] = version;
        data.set(payload, 1);
        const checksum = sha256(sha256(data)).slice(0, 4);
        const full = new Uint8Array(data.length + 4);
        full.set(data);
        full.set(checksum, data.length);
        return base58encode(full);
    }

    // ────────────── BECH32 ──────────────
    const BECH32_CHARSET = 'qpzry9x8gf2tvdw0s3jn54khce6mua7l';
    function bech32Polymod(values) {
        const GEN = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];
        let chk = 1;
        for (const v of values) {
            const b = chk >>> 25;
            chk = ((chk & 0x1ffffff) << 5) ^ v;
            for (let i = 0; i < 5; i++) if ((b >>> i) & 1) chk ^= GEN[i];
        }
        return chk;
    }
    function bech32HrpExpand(hrp) {
        const ret = [];
        for (const c of hrp) ret.push(c.charCodeAt(0) >> 5);
        ret.push(0);
        for (const c of hrp) ret.push(c.charCodeAt(0) & 31);
        return ret;
    }
    function bech32Encode(hrp, data, bech32m = false) {
        const values = bech32HrpExpand(hrp).concat(data);
        const CONST = bech32m ? 0x2bc830a3 : 1;
        const polymod = bech32Polymod(values.concat([0,0,0,0,0,0])) ^ CONST;
        const checksum = [];
        for (let i = 0; i < 6; i++) checksum.push((polymod >>> (5 * (5 - i))) & 31);
        let result = hrp + '1';
        for (const d of data.concat(checksum)) result += BECH32_CHARSET[d];
        return result;
    }
    function convertBits(data, fromBits, toBits, pad) {
        let acc = 0, bits = 0;
        const ret = [];
        const maxv = (1 << toBits) - 1;
        for (const v of data) {
            acc = (acc << fromBits) | v;
            bits += fromBits;
            while (bits >= toBits) { bits -= toBits; ret.push((acc >>> bits) & maxv); }
        }
        if (pad) { if (bits > 0) ret.push((acc << (toBits - bits)) & maxv); }
        else if (bits >= fromBits || ((acc << (toBits - bits)) & maxv)) throw new Error('Invalid bits');
        return ret;
    }
    function segwitEncode(hrp, witnessVersion, witnessProgram) {
        const data = [witnessVersion].concat(convertBits(witnessProgram, 8, 5, true));
        return bech32Encode(hrp, data, witnessVersion > 0);
    }

    // ────────────── ADDRESS VALIDATION ──────────────
    function validateAddress(addr) {
        addr = addr.trim();
        if (!addr) return { valid: false, error: 'Empty address' };

        // Try Bech32/Bech32m (bc1 / tb1)
        if (addr.toLowerCase().startsWith('bc1') || addr.toLowerCase().startsWith('tb1')) {
            return validateBech32Address(addr);
        }
        // Try Base58Check
        return validateBase58Address(addr);
    }

    function validateBech32Address(addr) {
        const lower = addr.toLowerCase();
        const hrp = lower.startsWith('bc1') ? 'bc' : 'tb';
        const network = hrp === 'bc' ? 'Bitcoin Mainnet' : 'Bitcoin Testnet';
        
        // Decode
        const sepIdx = lower.lastIndexOf('1');
        if (sepIdx < 1 || sepIdx + 7 > lower.length) return { valid: false, error: 'Invalid bech32 structure' };
        
        const dataPart = lower.slice(sepIdx + 1);
        const data = [];
        for (const c of dataPart) {
            const idx = BECH32_CHARSET.indexOf(c);
            if (idx === -1) return { valid: false, error: `Invalid bech32 character: ${c}` };
            data.push(idx);
        }
        
        const values = bech32HrpExpand(hrp).concat(data);
        const witnessVersion = data[0];
        const isBech32m = witnessVersion > 0;
        const CONST = isBech32m ? 0x2bc830a3 : 1;
        
        if (bech32Polymod(values) !== CONST) return { valid: false, error: 'Invalid bech32 checksum' };
        
        const payload = data.slice(1, -6);
        let program;
        try { program = convertBits(payload, 5, 8, false); } catch { return { valid: false, error: 'Invalid witness program' }; }
        
        let type, description, encoding;
        if (witnessVersion === 0 && program.length === 20) {
            type = 'Native SegWit (P2WPKH)';
            description = 'Pay-to-Witness-Public-Key-Hash — Segregated Witness v0 for single keys';
            encoding = 'Bech32';
        } else if (witnessVersion === 0 && program.length === 32) {
            type = 'Native SegWit (P2WSH)';
            description = 'Pay-to-Witness-Script-Hash — Segregated Witness v0 for scripts';
            encoding = 'Bech32';
        } else if (witnessVersion === 1 && program.length === 32) {
            type = 'Taproot (P2TR)';
            description = 'Pay-to-Taproot — Witness v1, Schnorr signatures, enhanced privacy';
            encoding = 'Bech32m';
        } else {
            type = `Witness v${witnessVersion}`;
            description = `Unknown witness version ${witnessVersion} program`;
            encoding = isBech32m ? 'Bech32m' : 'Bech32';
        }
        
        return {
            valid: true,
            address: addr,
            type,
            description,
            network,
            encoding,
            witnessVersion,
            programHex: toHex(new Uint8Array(program)),
            programLength: program.length,
            characteristics: getCharacteristics(type)
        };
    }

    function validateBase58Address(addr) {
        try {
            const decoded = base58decode(addr);
            if (decoded.length !== 25) return { valid: false, error: `Invalid length: expected 25 bytes, got ${decoded.length}` };
            
            const payload = decoded.slice(0, 21);
            const checksum = decoded.slice(21);
            const computed = sha256(sha256(payload)).slice(0, 4);
            
            for (let i = 0; i < 4; i++) {
                if (checksum[i] !== computed[i]) return { valid: false, error: 'Invalid Base58Check checksum' };
            }
            
            const version = payload[0];
            let type, description, network;
            
            if (version === 0x00) {
                type = 'Legacy (P2PKH)';
                description = 'Pay-to-Public-Key-Hash — Original Bitcoin address format';
                network = 'Bitcoin Mainnet';
            } else if (version === 0x05) {
                type = 'Script Hash (P2SH)';
                description = 'Pay-to-Script-Hash — Can wrap SegWit (P2SH-P2WPKH) or multisig scripts';
                network = 'Bitcoin Mainnet';
            } else if (version === 0x6f) {
                type = 'Testnet (P2PKH)';
                description = 'Pay-to-Public-Key-Hash — Testnet address';
                network = 'Bitcoin Testnet';
            } else if (version === 0xc4) {
                type = 'Testnet (P2SH)';
                description = 'Pay-to-Script-Hash — Testnet address';
                network = 'Bitcoin Testnet';
            } else {
                type = `Unknown (version: 0x${version.toString(16)})`;
                description = 'Unknown address version';
                network = 'Unknown';
            }
            
            return {
                valid: true,
                address: addr,
                type,
                description,
                network,
                encoding: 'Base58Check',
                versionByte: '0x' + version.toString(16).padStart(2, '0'),
                payloadHex: toHex(payload.slice(1)),
                characteristics: getCharacteristics(type)
            };
        } catch (e) {
            return { valid: false, error: e.message };
        }
    }

    function getCharacteristics(type) {
        if (type.includes('P2PKH') && !type.includes('Testnet')) return ['Starts with "1"', 'Most compatible (supported by all wallets)', 'Highest transaction fees', 'Base58Check encoding'];
        if (type.includes('P2SH') && !type.includes('Testnet')) return ['Starts with "3"', 'Can wrap SegWit for backward compatibility', 'Moderate transaction fees', 'Base58Check encoding'];
        if (type.includes('P2WPKH')) return ['Starts with "bc1q"', 'Lower fees (~38% savings vs P2PKH)', 'Native SegWit support', 'Bech32 encoding (lowercase)'];
        if (type.includes('P2TR') || type.includes('Taproot')) return ['Starts with "bc1p"', 'Lowest fees + best privacy', 'Schnorr signatures', 'Bech32m encoding', 'Enhanced scripting with MAST'];
        if (type.includes('Testnet')) return ['Testnet address — no real value', 'For testing and development only'];
        return [];
    }

    // ────────────── ADDRESS GENERATION FROM PUBKEY ──────────────
    function pubkeyToAddresses(hexPubkey) {
        hexPubkey = hexPubkey.trim();
        if (!/^(02|03)[0-9a-fA-F]{64}$/.test(hexPubkey)) {
            throw new Error('Invalid compressed public key. Must be 66 hex characters starting with 02 or 03.');
        }
        const pubkeyBytes = fromHex(hexPubkey);
        const h160 = hash160(pubkeyBytes);
        
        // P2PKH
        const p2pkh = base58checkEncode(0x00, h160);
        
        // P2SH-P2WPKH (nested SegWit)
        const witnessScript = new Uint8Array(22);
        witnessScript[0] = 0x00; // OP_0
        witnessScript[1] = 0x14; // push 20 bytes
        witnessScript.set(h160, 2);
        const scriptHash = hash160(witnessScript);
        const p2sh = base58checkEncode(0x05, scriptHash);
        
        // P2WPKH (native SegWit)
        const p2wpkh = segwitEncode('bc', 0, Array.from(h160));
        
        // P2TR (Taproot) — use the x-only pubkey (drop the 02/03 prefix)
        const xonly = pubkeyBytes.slice(1);
        const p2tr = segwitEncode('bc', 1, Array.from(xonly));
        
        return {
            pubkey: hexPubkey,
            pubkeyHash: toHex(h160),
            p2pkh: { address: p2pkh, type: 'Legacy (P2PKH)', prefix: '1', description: 'Original format. Most compatible, highest fees.' },
            p2sh:  { address: p2sh,  type: 'P2SH-SegWit',    prefix: '3', description: 'SegWit wrapped in P2SH for backward compatibility.' },
            p2wpkh:{ address: p2wpkh,type: 'Native SegWit (P2WPKH)', prefix: 'bc1q', description: 'Lower fees (~38% savings). Bech32 encoding.' },
            p2tr:  { address: p2tr,  type: 'Taproot (P2TR)',  prefix: 'bc1p', description: 'Lowest fees, best privacy. Schnorr signatures.' }
        };
    }

    // ────────────── MNEMONIC GENERATION ──────────────
    // BIP39 English wordlist (2048 words) - loaded dynamically
    let BIP39_WORDLIST = null;
    
    async function loadWordlist() {
        if (BIP39_WORDLIST) return BIP39_WORDLIST;
        // Fetch the standard BIP39 English wordlist
        try {
            const resp = await fetch('https://raw.githubusercontent.com/bitcoin/bips/master/bip-0039/english.txt');
            const text = await resp.text();
            BIP39_WORDLIST = text.trim().split('\n').map(w => w.trim());
            if (BIP39_WORDLIST.length !== 2048) throw new Error('Bad wordlist length');
            return BIP39_WORDLIST;
        } catch {
            throw new Error('Could not load BIP39 wordlist. Check your internet connection.');
        }
    }

    async function generateMnemonic(wordCount = 12) {
        const wordlist = await loadWordlist();
        const entropyBits = { 12: 128, 15: 160, 18: 192, 21: 224, 24: 256 }[wordCount];
        if (!entropyBits) throw new Error('Invalid word count');
        
        const entropyBytes = entropyBits / 8;
        const entropy = new Uint8Array(entropyBytes);
        crypto.getRandomValues(entropy);
        
        const hash = sha256(entropy);
        const checksumBits = entropyBits / 32;
        
        // Convert entropy + checksum to binary string
        let bits = '';
        for (const b of entropy) bits += b.toString(2).padStart(8, '0');
        for (let i = 0; i < checksumBits; i++) bits += ((hash[0] >> (7 - i)) & 1).toString();
        
        const words = [];
        for (let i = 0; i < wordCount; i++) {
            const idx = parseInt(bits.substr(i * 11, 11), 2);
            words.push(wordlist[idx]);
        }
        
        return {
            mnemonic: words.join(' '),
            words,
            wordCount,
            entropyHex: toHex(entropy),
            entropyBits,
            checksumBits
        };
    }

    // ────────────── PUBLIC API ──────────────
    return {
        validateAddress,
        pubkeyToAddresses,
        generateMnemonic,
        sha256: (d) => toHex(sha256(d)),
        hash160: (d) => toHex(hash160(typeof d === 'string' ? fromHex(d) : d)),
        toHex,
        fromHex
    };
})();
