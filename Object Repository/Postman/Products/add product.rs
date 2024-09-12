<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>add product</name>
   <tag></tag>
   <elementGuidId>a71a6c97-d900-4ca0-b81b-a619197d8f34</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.accessToken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;category_id\&quot; : \&quot;${categoryId}\&quot;,\n    \&quot;code\&quot;: \&quot;A314ASDDFIER3432\&quot;,\n    \&quot;name\&quot;: \&quot;${nama}\&quot;,\n    \&quot;price\&quot;: \&quot;3500\&quot;,\n    \&quot;cost\&quot;: \&quot;3000\&quot;,\n    \&quot;stock\&quot;: \&quot;1\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>c2319784-80b1-43c8-a130-94a144e033a0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.accessToken}</value>
      <webElementGuid>344ce884-a861-4c52-a727-1488d9a10aff</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/products</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>ccbba59b-4086-431a-8b14-7c35f9df0800</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.categoryId</defaultValue>
      <description></description>
      <id>c89a43c4-1c5d-4f66-a9d8-68faebde0505</id>
      <masked>false</masked>
      <name>categoryId</name>
   </variables>
   <variables>
      <defaultValue>'Lays'</defaultValue>
      <description></description>
      <id>d9c225c8-6ac4-4d6b-b6a6-0a1c465116e5</id>
      <masked>false</masked>
      <name>nama</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)
WS.verifyElementPropertyValue(response, 'data.name','Lays' )</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
