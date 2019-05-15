<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateProductStatus</name>
   <tag></tag>
   <elementGuidId>1a0336f2-88cc-4b7f-bda8-f8be1940a664</elementGuidId>
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
      &lt;aom:UpdateProductStatus>
         &lt;pReqInfo>
            &lt;ACTION_INTERACTION_TYPE_ID>2&lt;/ACTION_INTERACTION_TYPE_ID>
            &lt;ACTION_REASON_CODE>PORT_IN&lt;/ACTION_REASON_CODE>
            &lt;ACTION_DATE>2019/04/26-18:00:00&lt;/ACTION_DATE>
            &lt;TRANSACTION_ID>YY1651561&lt;/TRANSACTION_ID>
            &lt;CUSTOMER_ID>${custID}&lt;/CUSTOMER_ID>
            &lt;USER_NAME>BSCS&lt;/USER_NAME>
            &lt;CHANNEL_NAME>cansu&lt;/CHANNEL_NAME>
            &lt;MODIFY_INFO>
               &lt;CREATE_INFO>
                  &lt;CREATE_DATE>2019/04/26-18:00:00&lt;/CREATE_DATE>
                  &lt;CREATE_USER>BSCS&lt;/CREATE_USER>
               &lt;/CREATE_INFO>
            &lt;/MODIFY_INFO>
         &lt;/pReqInfo>
         &lt;!--1 or more repetitions:-->
         &lt;pKeyArray>
           &lt;PRODUCT_ID>${productID}&lt;/PRODUCT_ID>
         &lt;/pKeyArray>
         &lt;pStatus>${productStatus}&lt;/pStatus>
      &lt;/aom:UpdateProductStatus>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>UpdateProductStatus</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.custID</defaultValue>
      <description></description>
      <id>1371e22d-976f-4d24-88f4-3fdb95d18f57</id>
      <masked>false</masked>
      <name>custID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.productStatus</defaultValue>
      <description></description>
      <id>fa3631c9-4f48-447a-b306-c1b9775486bf</id>
      <masked>false</masked>
      <name>productStatus</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.productID</defaultValue>
      <description></description>
      <id>625e6411-c629-4b1e-8a28-d6c5954c0883</id>
      <masked>false</masked>
      <name>productID</name>
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
   <wsdlAddress>http://172.30.10.32:8181/AOMWSProduct/AOMWSProduct?wsdl</wsdlAddress>
</WebServiceRequestEntity>
