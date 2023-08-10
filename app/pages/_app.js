import Head from 'next/head'
import '../styles/global.css'
// Import WalletConnectionProvider from components
// Import the solana wallet css

function MyApp({ Component, pageProps }) {
    return (
        <>
            <Head>
                <title>Todo</title>
            </Head>
            <main>
                <Component {...pageProps} />
            </main>
        </>
    )
}

export default MyApp
