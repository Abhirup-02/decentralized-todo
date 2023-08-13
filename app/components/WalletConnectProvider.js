import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react'
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui'
import { GloWalletAdapter, PhantomWalletAdapter, SlopeWalletAdapter } from '@solana/wallet-adapter-wallets'
import { clusterApiUrl } from '@solana/web3.js'
import { useMemo } from 'react'


export const WalletConnectProvider = ({ children }) => {
    const network = WalletAdapterNetwork.Testnet

    const endpoint = useMemo(() => {
        if (network === WalletAdapterNetwork.Testnet) {
            return 'https://green-wiser-violet.solana-testnet.discover.quiknode.pro/ba1187ecaae779be46900d5f0355ac4ff2780bc7/'
        }
        return clusterApiUrl(network)
    }, [network])

    const wallets = useMemo(() => [new PhantomWalletAdapter()], [network])

    return (
        <ConnectionProvider endpoint={endpoint}>
            <WalletProvider wallets={wallets} autoConnect>
                <WalletModalProvider>{children}</WalletModalProvider>
            </WalletProvider>
        </ConnectionProvider >
    )
}