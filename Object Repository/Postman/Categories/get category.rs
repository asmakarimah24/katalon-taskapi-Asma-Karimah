<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get category</name>
   <tag></tag>
   <elementGuidId>ecef0811-2b03-4724-b6eb-1903fa955f22</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjkyMDA4ODc4LTM5ZDEtNGE4NC04MmIxLTY2ODhlNGVhYTg4NiIsImNvbXBhbnlJZCI6IjNhNTQ4YTg4LThmMWUtNGE0ZS05NjAzLTA2NjdhZmZhMjI0OSIsImlhdCI6MTcyNTc5MzIzMH0._CkeYgCq1cdgqncUKJi93SwnnelMYQesu594ZIObSgs</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjkyMDA4ODc4LTM5ZDEtNGE4NC04MmIxLTY2ODhlNGVhYTg4NiIsImNvbXBhbnlJZCI6IjNhNTQ4YTg4LThmMWUtNGE0ZS05NjAzLTA2NjdhZmZhMjI0OSIsImlhdCI6MTcyNTc5MzIzMH0._CkeYgCq1cdgqncUKJi93SwnnelMYQesu594ZIObSgs</value>
      <webElementGuid>156b5033-1dbf-4f6a-b34e-957af2c36104</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${host}/categories/086ca5c3-330e-4e93-881e-1d057342b749</restUrl>
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
      <id>093e63c9-2af5-4ff1-99b2-903c52808729</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.categoryId</defaultValue>
      <description></description>
      <id>0689794e-154c-4aa1-aa38-6f2c08b8a018</id>
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
