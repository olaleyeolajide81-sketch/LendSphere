import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import * as freighter from '@stellar/freighter-api';
import { Server, Networks, TransactionBuilder, Operation } from '@stellar/stellar-sdk';

interface WalletContextType {
  isConnected: boolean;
  address: string | null;
  balance: string;
  connect: () => Promise<void>;
  disconnect: () => void;
  signTransaction: (xdr: string) => Promise<string>;
  isLoading: boolean;
  error: string | null;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  return context;
};

interface WalletProviderProps {
  children: ReactNode;
}

export const WalletProvider: React.FC<WalletProviderProps> = ({ children }) => {
  const [isConnected, setIsConnected] = useState(false);
  const [address, setAddress] = useState<string | null>(null);
  const [balance, setBalance] = useState('0');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const server = new Server('https://horizon-testnet.stellar.org');

  const connect = async () => {
    try {
      setIsLoading(true);
      setError(null);

      if (!window.freighter) {
        throw new Error('Freighter wallet is not installed');
      }

      const isConnectedResult = await window.freighter.isConnected();
      if (!isConnectedResult.isConnected) {
        throw new Error('Please connect your Freighter wallet');
      }

      const publicKey = await window.freighter.getPublicKey();
      if (!publicKey) {
        throw new Error('Failed to get public key');
      }

      setAddress(publicKey);
      setIsConnected(true);

      // Fetch balance
      const account = await server.loadAccount(publicKey);
      const nativeBalance = account.balances.find(
        (balance) => balance.asset_type === 'native'
      );
      setBalance(nativeBalance ? (nativeBalance as any).balance : '0');

    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to connect wallet');
      console.error('Wallet connection error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const disconnect = () => {
    setIsConnected(false);
    setAddress(null);
    setBalance('0');
    setError(null);
  };

  const signTransaction = async (xdr: string): Promise<string> => {
    try {
      if (!window.freighter) {
        throw new Error('Freighter wallet is not installed');
      }

      const signedResult = await window.freighter.signTransaction(xdr, Networks.TESTNET);
      if (!signedResult.signedXdr) {
        throw new Error('Failed to sign transaction');
      }

      return signedResult.signedXdr;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to sign transaction';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  };

  useEffect(() => {
    // Check if wallet is already connected on mount
    const checkConnection = async () => {
      if (window.freighter) {
        try {
          const isConnectedResult = await window.freighter.isConnected();
          if (isConnectedResult.isConnected) {
            const publicKey = await window.freighter.getPublicKey();
            if (publicKey) {
              setAddress(publicKey);
              setIsConnected(true);

              // Fetch balance
              const account = await server.loadAccount(publicKey);
              const nativeBalance = account.balances.find(
                (balance) => balance.asset_type === 'native'
              );
              setBalance(nativeBalance ? (nativeBalance as any).balance : '0');
            }
          }
        } catch (err) {
          console.error('Failed to check wallet connection:', err);
        }
      }
    };

    checkConnection();
  }, []);

  const value: WalletContextType = {
    isConnected,
    address,
    balance,
    connect,
    disconnect,
    signTransaction,
    isLoading,
    error,
  };

  return (
    <WalletContext.Provider value={value}>
      {children}
    </WalletContext.Provider>
  );
};

// Extend Window interface for Freighter
declare global {
  interface Window {
    freighter?: {
      isConnected: () => Promise<{ isConnected: boolean }>;
      getPublicKey: () => Promise<string>;
      signTransaction: (xdr: string, network: string) => Promise<{ signedXdr: string }>;
    };
  }
}
