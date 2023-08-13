import Head from 'next/head'
import '../styles/global.css'
// Import WalletConnectionProvider from components
import { WalletConnectProvider } from '../components/WalletConnectProvider'
// Import the solana wallet css
import '@solana/wallet-adapter-react-ui/styles.css'

function MyApp({ Component, pageProps }) {
    return (
        <>
            <Head>
                <title>Todo</title>
            </Head>
            <main>
                <WalletConnectProvider>
                    <Component {...pageProps} />
                </WalletConnectProvider>
            </main>
        </>
    )
}

export default MyApp
