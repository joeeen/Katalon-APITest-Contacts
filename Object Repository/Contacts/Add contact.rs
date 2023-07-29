<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add contact</name>
   <tag></tag>
   <elementGuidId>5d451afe-e6ee-4503-9031-909a45eb0943</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJfaWQiOiI2NGM0ZDFlMDBhM2VkZjAwMTMxYzllNWMiLCJpYXQiOjE2OTA2MjAzODR9.ZNqJheMDhty_iJakzNKLASjgGqVILiySRyEgBaC6x1E</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstName\&quot;: \&quot;John\&quot;,\n    \&quot;lastName\&quot;: \&quot;Doe\&quot;,\n    \&quot;birthdate\&quot;: \&quot;1970-01-01\&quot;,\n    \&quot;email\&quot;: \&quot;jdoe@fake.com\&quot;,\n    \&quot;phone\&quot;: \&quot;8005555555\&quot;,\n    \&quot;street1\&quot;: \&quot;1 Main St.\&quot;,\n    \&quot;street2\&quot;: \&quot;Apartment A\&quot;,\n    \&quot;city\&quot;: \&quot;Anytown\&quot;,\n    \&quot;stateProvince\&quot;: \&quot;KS\&quot;,\n    \&quot;postalCode\&quot;: \&quot;12345\&quot;,\n    \&quot;country\&quot;: \&quot;USA\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>dfba8070-ffd6-429d-a73e-24fa4dff83d9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJfaWQiOiI2NGM0ZDFlMDBhM2VkZjAwMTMxYzllNWMiLCJpYXQiOjE2OTA2MjAzODR9.ZNqJheMDhty_iJakzNKLASjgGqVILiySRyEgBaC6x1E</value>
      <webElementGuid>ffbb6271-9e56-4eca-9585-e6be524dd394</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/contacts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()





WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
