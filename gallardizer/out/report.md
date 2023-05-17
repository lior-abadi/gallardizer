# Issues Summary
## Low Risk Issues
Total: 42 instances were found over 1 issue\.





# Low Risk Issues
## \[L\-1\] Insecure declaration of pragma version
The specified <code>pragma</code> version allows for the utilization of different compiler versions to compile the source code\.
It's important to consider the potential risks associated with using a floating or flexible pragma version\. 
For instance, employing versions <code>0\.8\.7</code> or earlier may result in compilation errors, as they lack support for 
functions overriding interface functions without using the <code>override</code> modifier, 
which is exclusively available in Solidity <code>0\.8\.8</code> and newer versions\.<br> 

Similarly, the usage of <code>abi\.encodeCall</code>, which was introduced in Solidity <code>0\.8\.11</code>, 
may cause issues if the codebase relies on it\. Although it is uncertain whether these specific bugs related to <code>override</code> 
or <code>encode</code> will manifest in the code, exercising caution is advised to avoid potential unexpected scenarios or compatibility
issues that may arise with the inclusion of new features or implementations\.
Considering the uncertainty of potential bugs related to <code>override</code>, <code>encode</code>, or others, using a floating \(flexible\)
<code>pragma</code> version might lead to the project compiling with uncertain versions within that range\.<br>

Consider upgrading the pragma version to a newer release, preferably the most recent version available, 
in order to mitigate potential risks stemming from bug fixes introduced in previous releases\. 
Additionally, it is recommended to make the pragma version fixed to ensure consistency and stability in the project\.

*This issue was found 42 times:*

pragma solidity ^0\.8\.0;




pragma solidity >=0\.8\.17;




pragma solidity ^0\.8\.9;




pragma solidity ^0\.8\.6;




pragma solidity ^0\.8\.0;




pragma solidity ^0\.8\.0;




pragma solidity =0\.7\.6;




pragma solidity ^0\.8\.17;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity ^0\.8\.10;




pragma solidity >=0\.7\.6;




pragma solidity >=0\.8\.16;




pragma solidity >=0\.8\.17;




pragma solidity >=0\.8\.16;




pragma solidity >=0\.7\.6;











