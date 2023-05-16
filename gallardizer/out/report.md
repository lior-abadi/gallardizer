## \[L\-1\] Inconsistent floating pragma version
The specified <code>pragma</code> version allows for the utilization of compiler versions
beyond <code>0\.8\.0</code> to compile the source code\.
However, it's important to consider the potential risks associated with using a floating pragma version\.<br>

Employing versions <code>0\.8\.7</code>or earlier may result in compilation errors, as they lack support for
functions overriding interface functions without using the <code>override</code> modifier, which is exclusively 
available in Solidity <code>0\.8\.8</code> and newer versions\. Similarly, the usage of abi\.encodeCall, 
which was introduced in Solidity <code>0\.8\.11</code>, may cause issues if the codebase relies on it\.<br>

While it is not confirmed whether these specific bugs related to override or encoding will appear in the code, 
it is advised to be cautious\. Considering the uncertainty of potential bugs related to 
<code>override</code>, <code>encode</code>, or others, it is recommended to avoid using a floating pragma version\.<br>

Consider upgrading the pragma version to a newer release the most recent version available, 
in order to mitigate potential risks leveraging from bug fixes introduced on newer releases\. 
Also, make the pragma version fixed\.

*This issue was found 2 times:*

