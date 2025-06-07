import Overview from "./routes/Overview.svelte";
import Borrow from "./routes/Borrow.svelte";
import Pool from "./routes/Pool.svelte";
import Vaults from "./routes/Vaults.svelte";
import Staking from "./routes/Staking.svelte";
import Faucet from "./components/Faucet.svelte";
import Governance from "./routes/Governance.svelte";
export default {
    '/': Overview,
    '/borrow':Borrow,
    '/borrow/:param':Borrow,
    '/pool':Pool,
    '/vaults':Vaults,
    '/staking':Staking,
    '/faucet':Faucet,
    '/governance':Governance
};
