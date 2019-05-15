<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateCustomer</name>
   <tag></tag>
   <elementGuidId>4f146e9d-2a3b-426a-b35c-9e6bae2c879c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:aom=&quot;http://www.i2i.com/fcbs/soa/schemas/AOMWS&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;aom:UpdateCustomer>
         &lt;pReqInfo>
            &lt;ACTION_INTERACTION_TYPE_ID>143&lt;/ACTION_INTERACTION_TYPE_ID>
            &lt;ACTION_DATE>${updateCustomerDate}&lt;/ACTION_DATE>
            &lt;TRANSACTION_ID>CA147852&lt;/TRANSACTION_ID>
            &lt;CUSTOMER_ID>1262104&lt;/CUSTOMER_ID>
            &lt;USER_NAME>cansuarslan&lt;/USER_NAME>
            &lt;CHANNEL_NAME>cansuarslan&lt;/CHANNEL_NAME>
            &lt;SIMULATION_FLAG>false&lt;/SIMULATION_FLAG>
            &lt;MODIFY_INFO>
               &lt;CREATE_INFO>
                  &lt;CREATE_DATE>${updateCustomerDate}&lt;/CREATE_DATE>
                  &lt;CREATE_USER>cansuarslan&lt;/CREATE_USER>
               &lt;/CREATE_INFO>
            &lt;/MODIFY_INFO>
         &lt;/pReqInfo>
         &lt;pCustInfo>
            &lt;CUSTOMER_KEY>
               &lt;CUSTOMER_ID>${custID}&lt;/CUSTOMER_ID>
            &lt;/CUSTOMER_KEY>
            &lt;CUSTOMER_DEFINITION>
               &lt;CUSTOMER_NAME>${customerName}&lt;/CUSTOMER_NAME>
               &lt;CUSTOMER_STATUS>${customerStatus}&lt;/CUSTOMER_STATUS>
               &lt;CUSTOMER_TYPE>${customerType}&lt;/CUSTOMER_TYPE>
               &lt;SEGMENT_CODE>${segmentCode}&lt;/SEGMENT_CODE>
            &lt;/CUSTOMER_DEFINITION>
         &lt;/pCustInfo>
      &lt;/aom:UpdateCustomer>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>UpdateCustomer</soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>faa144f9-9a21-4e47-8ba7-76241d0af86a</id>
      <masked>false</masked>
      <name>custID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>30886353-f3f0-4c3c-985a-08b23175ddbe</id>
      <masked>false</masked>
      <name>segmentCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5e1342b5-ca76-4e90-a124-689a1c669f67</id>
      <masked>false</masked>
      <name>updateCustomerDate</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>67d4999c-c1a0-4376-91f9-78fa1e3989f8</id>
      <masked>false</masked>
      <name>customerType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>daf693da-f92e-444c-b864-cc40413ed22b</id>
      <masked>false</masked>
      <name>customerName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5e4df1a9-4b8f-4d63-9a29-35cd87218101</id>
      <masked>false</masked>
      <name>customerStatus</name>
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
</verificationScript>
   <wsdlAddress>http://172.30.10.32:8181/AOMWSCustomer/AOMWSCustomer?wsdl</wsdlAddress>
</WebServiceRequestEntity>
