import React from 'react';
import { motion } from 'framer-motion';
import { useWallet } from '../contexts/WalletContext';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/Card';
import { Button } from '../components/ui/Button';
import { Wallet, TrendingUp, TrendingDown, DollarSign } from 'lucide-react';

const Dashboard: React.FC = () => {
  const { isConnected, address, balance, connect, disconnect } = useWallet();

  const containerVariants = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: {
        staggerChildren: 0.1,
      },
    },
  };

  const itemVariants = {
    hidden: { y: 20, opacity: 0 },
    visible: {
      y: 0,
      opacity: 1,
    },
  };

  if (!isConnected) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
          className="text-center"
        >
          <Wallet className="w-16 h-16 mx-auto mb-4 text-blue-500" />
          <h1 className="text-3xl font-bold mb-4">Welcome to LendSphere</h1>
          <p className="text-gray-600 mb-6">Connect your wallet to start lending and borrowing</p>
          <Button onClick={connect} className="px-6 py-3">
            Connect Wallet
          </Button>
        </motion.div>
      </div>
    );
  }

  return (
    <motion.div
      variants={containerVariants}
      initial="hidden"
      animate="visible"
      className="space-y-6"
    >
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold">Dashboard</h1>
          <p className="text-gray-600">Manage your lending and borrowing positions</p>
        </div>
        <div className="text-sm text-gray-500">
          Connected: {address?.slice(0, 6)}...{address?.slice(-4)}
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <motion.div variants={itemVariants}>
          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600">Net Worth</p>
                  <p className="text-2xl font-bold">$12,345.67</p>
                </div>
                <DollarSign className="w-8 h-8 text-green-500" />
              </div>
            </CardContent>
          </Card>
        </motion.div>

        <motion.div variants={itemVariants}>
          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600">Supply Balance</p>
                  <p className="text-2xl font-bold">$8,234.56</p>
                </div>
                <TrendingUp className="w-8 h-8 text-blue-500" />
              </div>
            </CardContent>
          </Card>
        </motion.div>

        <motion.div variants={itemVariants}>
          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600">Borrow Balance</p>
                  <p className="text-2xl font-bold">$2,345.67</p>
                </div>
                <TrendingDown className="w-8 h-8 text-red-500" />
              </div>
            </CardContent>
          </Card>
        </motion.div>

        <motion.div variants={itemVariants}>
          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-600">Health Factor</p>
                  <p className="text-2xl font-bold text-green-500">1.85</p>
                </div>
                <div className="w-8 h-8 rounded-full bg-green-100 flex items-center justify-center">
                  <div className="w-4 h-4 rounded-full bg-green-500"></div>
                </div>
              </div>
            </CardContent>
          </Card>
        </motion.div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <motion.div variants={itemVariants}>
          <Card>
            <CardHeader>
              <CardTitle>Your Positions</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div className="flex justify-between items-center p-3 border rounded-lg">
                  <div>
                    <p className="font-medium">XLM</p>
                    <p className="text-sm text-gray-600">Supplied: 5,000 XLM</p>
                  </div>
                  <div className="text-right">
                    <p className="font-medium">$1,234.56</p>
                    <p className="text-sm text-green-600">+5.2% APY</p>
                  </div>
                </div>
                <div className="flex justify-between items-center p-3 border rounded-lg">
                  <div>
                    <p className="font-medium">USDC</p>
                    <p className="text-sm text-gray-600">Borrowed: 1,000 USDC</p>
                  </div>
                  <div className="text-right">
                    <p className="font-medium">$1,000.00</p>
                    <p className="text-sm text-red-600">+8.5% APY</p>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </motion.div>

        <motion.div variants={itemVariants}>
          <Card>
            <CardHeader>
              <CardTitle>Recent Activity</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-600">Supplied XLM</span>
                  <span className="font-medium">+1,000 XLM</span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-600">Borrowed USDC</span>
                  <span className="font-medium">-500 USDC</span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-600">Interest Earned</span>
                  <span className="font-medium text-green-600">+$12.34</span>
                </div>
              </div>
            </CardContent>
          </Card>
        </motion.div>
      </div>
    </motion.div>
  );
};

export default Dashboard;
