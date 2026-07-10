const vault = {
  totalAssets: 0,
  totalShares: 0,
  users: {}
};

function deposit(user, amount) {
  let shares;
  
  if (vault.totalShares === 0) {
    shares = amount;
  } else {
    shares = (amount * vault.totalShares) / vault.totalAssets;
  }
  
  vault.totalAssets += amount;
  vault.totalShares += shares;
  vault.users[user] = (vault.users[user] || 0) + shares;
  
  return shares;
}

function withdraw(user, shares) {
  const assets = (shares * vault.totalAssets) / vault.totalShares;
  
  vault.totalShares -= shares;
  vault.totalAssets -= assets;
  vault.users[user] -= shares;
  
  return assets;
}

function addYield(amount) {
  vault.totalAssets += amount;
}
