<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update contact (put)</name>
   <tag></tag>
   <elementGuidId>383e7ec0-aaed-4489-84be-85f172a56728</elementGuidId>
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
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstName\&quot;: \&quot;Amy\&quot;,\n    \&quot;lastName\&quot;: \&quot;Miller\&quot;,\n    \&quot;birthdate\&quot;: \&quot;1992-02-02\&quot;,\n    \&quot;email\&quot;: \&quot;amiller@fake.com\&quot;,\n    \&quot;phone\&quot;: \&quot;8005554242\&quot;,\n    \&quot;street1\&quot;: \&quot;13 School St.\&quot;,\n    \&quot;street2\&quot;: \&quot;Apt. 5\&quot;,\n    \&quot;city\&quot;: \&quot;Washington\&quot;,\n    \&quot;stateProvince\&quot;: \&quot;QC\&quot;,\n    \&quot;postalCode\&quot;: \&quot;A1A1A1\&quot;,\n    \&quot;country\&quot;: \&quot;Canada\&quot;\n}&quot;,
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
      <webElementGuid>8b56f713-8425-4fee-8efb-b2b65b3f7ed3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJfaWQiOiI2NGM0ZDFlMDBhM2VkZjAwMTMxYzllNWMiLCJpYXQiOjE2OTA2MjAzODR9.ZNqJheMDhty_iJakzNKLASjgGqVILiySRyEgBaC6x1E</value>
      <webElementGuid>109fd326-7789-4005-813c-88bd1c8d459e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/contacts/{{contactId}}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>