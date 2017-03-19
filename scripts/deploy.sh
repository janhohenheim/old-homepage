git pull
rm -f rpm/x86_64/jnferner-*.rpm
./scripts/package.sh
ssh root@jnferner.com rm -f 'jnferner-*.rpm'
scp rpm/x86_64/jnferner-*.rpm root@jnferner.com:~/
ssh root@jnferner.com rpm -Uhv 'jnferner-*.rpm'
