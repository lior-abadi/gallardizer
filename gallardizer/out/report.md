## \[L\-1\] Inconsistent floating pragma version
The specified \`pragma\` version allows for the utilization of compiler versions beyond 0\.8\.0 to compile the source code\.
However, it's important to consider the potential risks associated with using a floating pragma version\.<br>

Employing versions 0\.8\.7 or earlier may result in compilation errors, as they lack support for functions overriding interface 
functions without using the 'override' keyword, which is exclusively available in Solidity 0\.8\.8 and newer versions\. 
Similarly, the usage of abi\.encodeCall, which was introduced in Solidity 0\.8\.11, may cause issues if the codebase relies on it\.<br>

While it is not confirmed whether these specific bugs related to override or encoding will appear in the code, it is advised to be cautious\. 
Considering the uncertainty of potential bugs related to override, encoding, or others, it is recommended to avoid using a floating pragma version\.<br>

Consider upgrading the pragma version to a newer release the most recent version available, 
in order to mitigate potential risks leveraging from bug fixes introduced on newer releases\. 
Also, make the pragma version fixed\.

*This issue was found 2 times:*

