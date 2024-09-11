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
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjkyMDA4ODc4LTM5ZDEtNGE4NC04MmIxLTY2ODhlNGVhYTg4NiIsImNvbXBhbnlJZCI6IjNhNTQ4YTg4LThmMWUtNGE0ZS05NjAzLTA2NjdhZmZhMjI0OSIsImlhdCI6MTcyNTc5NDAzN30.S0RdopLZCjBoTu6GK09XazHk6tpWcJmjSPOhDFr9z3Y</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;category_id\&quot; : \&quot;${categoryId}\&quot;,\n    \&quot;code\&quot;: \&quot;A314ASDDFIER3432\&quot;,\n    \&quot;name\&quot;: \&quot;taro\&quot;,\n    \&quot;price\&quot;: \&quot;3500\&quot;,\n    \&quot;cost\&quot;: \&quot;3000\&quot;,\n    \&quot;stock\&quot;: \&quot;1\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjkyMDA4ODc4LTM5ZDEtNGE4NC04MmIxLTY2ODhlNGVhYTg4NiIsImNvbXBhbnlJZCI6IjNhNTQ4YTg4LThmMWUtNGE0ZS05NjAzLTA2NjdhZmZhMjI0OSIsImlhdCI6MTcyNTc5NDAzN30.S0RdopLZCjBoTu6GK09XazHk6tpWcJmjSPOhDFr9z3Y</value>
      <webElementGuid>410fa8ec-5b52-45e3-948d-604d2404db60</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
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
   <socketTimeout>-1</socketTimeout>
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
