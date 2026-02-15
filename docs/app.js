/**
 * app.js — UI logic for Bitcoin Address Tools web interface.
 */
document.addEventListener('DOMContentLoaded', () => {
    // ────────── TAB NAVIGATION ──────────
    const tabs = document.querySelectorAll('.tab');
    const contents = document.querySelectorAll('.tab-content');

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            tabs.forEach(t => { t.classList.remove('active'); t.setAttribute('aria-selected', 'false'); });
            contents.forEach(c => c.classList.remove('active'));
            tab.classList.add('active');
            tab.setAttribute('aria-selected', 'true');
            document.getElementById('tab-' + tab.dataset.tab).classList.add('active');
        });
    });

    // ────────── EXAMPLE BUTTONS ──────────
    document.querySelectorAll('.example-btn:not(.convert-example)').forEach(btn => {
        btn.addEventListener('click', () => {
            document.getElementById('validate-input').value = btn.dataset.value;
            doValidate();
        });
    });
    document.querySelectorAll('.convert-example').forEach(btn => {
        btn.addEventListener('click', () => {
            document.getElementById('convert-input').value = btn.dataset.value;
            doConvert();
        });
    });

    // ────────── VALIDATE ──────────
    document.getElementById('validate-btn').addEventListener('click', doValidate);
    document.getElementById('validate-input').addEventListener('keydown', e => { if (e.key === 'Enter') doValidate(); });

    function doValidate() {
        const input = document.getElementById('validate-input').value.trim();
        const resultArea = document.getElementById('validate-result');
        if (!input) { resultArea.innerHTML = ''; return; }

        const result = Bitcoin.validateAddress(input);

        if (result.valid) {
            let rows = '';
            rows += row('Address', result.address);
            rows += row('Type', result.type, 'highlight');
            rows += row('Description', result.description);
            rows += row('Network', result.network);
            rows += row('Encoding', result.encoding);
            if (result.witnessVersion !== undefined) rows += row('Witness Version', 'v' + result.witnessVersion);
            if (result.versionByte) rows += row('Version Byte', result.versionByte);
            if (result.programHex) rows += row('Program (hex)', result.programHex);
            if (result.programLength) rows += row('Program Length', result.programLength + ' bytes');
            if (result.payloadHex) rows += row('Payload (hex)', result.payloadHex);

            let chars = '';
            if (result.characteristics && result.characteristics.length) {
                chars = '<div style="margin-top:1rem;"><strong style="color:#f7931a;">💡 Characteristics</strong><ul style="margin:0.5rem 0 0 1.2rem;color:#999;font-size:0.88rem;">';
                result.characteristics.forEach(c => chars += `<li>${c}</li>`);
                chars += '</ul></div>';
            }

            resultArea.innerHTML = `<div class="result-card success"><h4>✅ Valid Address</h4>${rows}${chars}</div>`;
        } else {
            resultArea.innerHTML = `<div class="result-card error">
                <h4>❌ Invalid Address</h4>
                ${row('Input', input)}
                ${row('Error', result.error)}
                <div style="margin-top:1rem;color:#888;font-size:0.85rem;">
                    <strong>💡 Common Issues:</strong>
                    <ul style="margin:0.5rem 0 0 1.2rem;">
                        <li>Check for typos or missing characters</li>
                        <li>Ensure the address starts with 1, 3, bc1q, or bc1p (mainnet)</li>
                        <li>Testnet addresses start with m, n, 2, or tb1</li>
                    </ul>
                </div>
            </div>`;
        }
    }

    // ────────── GENERATE WALLET ──────────
    document.getElementById('generate-btn').addEventListener('click', doGenerate);

    async function doGenerate() {
        const btn = document.getElementById('generate-btn');
        const resultArea = document.getElementById('generate-result');
        const wordCount = parseInt(document.getElementById('word-count').value);
        const network = document.getElementById('wallet-network').value;

        btn.textContent = 'Generating...';
        btn.disabled = true;

        try {
            const result = await Bitcoin.generateMnemonic(wordCount);

            let wordGrid = '<div class="mnemonic-grid">';
            result.words.forEach((w, i) => {
                wordGrid += `<div class="mnemonic-word"><span>${i + 1}.</span>${w}</div>`;
            });
            wordGrid += '</div>';

            const path = network === 'testnet' ? "m/44'/1'/0'/0/0" : "m/44'/0'/0'/0/0";

            resultArea.innerHTML = `<div class="result-card success">
                <h4>🎲 New HD Wallet Generated</h4>
                <div style="background:rgba(244,67,54,0.08);border:1px solid rgba(244,67,54,0.2);border-radius:8px;padding:0.8rem 1rem;margin-bottom:1rem;font-size:0.85rem;color:#f44336;">
                    ⚠️ <strong>NEVER</strong> use this mnemonic for real funds! This is for educational purposes only.
                </div>
                <strong style="color:#f7931a;font-size:0.95rem;">🌱 Mnemonic Phrase (${result.wordCount} words)</strong>
                ${wordGrid}
                ${row('Entropy', result.entropyHex)}
                ${row('Entropy Bits', result.entropyBits + ' bits')}
                ${row('Checksum Bits', result.checksumBits + ' bits')}
                ${row('Network', network === 'testnet' ? 'Bitcoin Testnet' : 'Bitcoin Mainnet')}
                ${row('Derivation Path', path)}
                <div style="margin-top:1rem;color:#888;font-size:0.85rem;">
                    <strong>💡 How it works:</strong>
                    <ul style="margin:0.5rem 0 0 1.2rem;">
                        <li>Random entropy is generated using your browser's secure RNG</li>
                        <li>A SHA-256 checksum is appended</li>
                        <li>The bits are split into 11-bit groups and mapped to the BIP39 wordlist</li>
                        <li>The mnemonic can be used to derive an infinite number of keys</li>
                    </ul>
                </div>
            </div>`;
        } catch (e) {
            resultArea.innerHTML = `<div class="result-card error"><h4>❌ Error</h4>${row('Error', e.message)}</div>`;
        } finally {
            btn.textContent = 'Generate Wallet';
            btn.disabled = false;
        }
    }

    // ────────── CONVERT ──────────
    document.getElementById('convert-btn').addEventListener('click', doConvert);
    document.getElementById('convert-input').addEventListener('keydown', e => { if (e.key === 'Enter') doConvert(); });

    function doConvert() {
        const input = document.getElementById('convert-input').value.trim();
        const resultArea = document.getElementById('convert-result');
        if (!input) { resultArea.innerHTML = ''; return; }

        try {
            const result = Bitcoin.pubkeyToAddresses(input);
            
            const formats = [result.p2pkh, result.p2sh, result.p2wpkh, result.p2tr];
            let cards = '';
            const icons = ['🟡', '🔵', '🟢', '🟣'];
            formats.forEach((fmt, i) => {
                cards += `<div class="format-card">
                    <h5>${icons[i]} ${fmt.type}</h5>
                    <div class="addr">${fmt.address}</div>
                    <div class="desc">${fmt.description}</div>
                </div>`;
            });

            resultArea.innerHTML = `<div class="result-card success">
                <h4>🔄 All Address Formats</h4>
                ${row('Public Key', result.pubkey)}
                ${row('Hash160', result.pubkeyHash)}
                <div style="margin-top:1rem;">${cards}</div>
            </div>`;
        } catch (e) {
            resultArea.innerHTML = `<div class="result-card error"><h4>❌ Error</h4>${row('Error', e.message)}</div>`;
        }
    }

    // ────────── HELPER ──────────
    function row(label, value, cls = '') {
        return `<div class="result-row"><div class="result-label">${label}</div><div class="result-value ${cls}">${escapeHtml(String(value))}</div></div>`;
    }
    function escapeHtml(s) {
        return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
    }
});
