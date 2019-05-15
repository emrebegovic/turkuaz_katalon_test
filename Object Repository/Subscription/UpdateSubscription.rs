<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UpdateSubscription</name>
   <tag></tag>
   <elementGuidId>dd3b52ba-1e1d-4be2-8764-fc491890eb54</elementGuidId>
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
      &lt;aom:UpdateSubscription>
          &lt;pReqInfo>
            &lt;ACTION_INTERACTION_TYPE_ID>2&lt;/ACTION_INTERACTION_TYPE_ID>
            &lt;ACTION_REASON_CODE>PORT_IN&lt;/ACTION_REASON_CODE>
            &lt;ACTION_DATE>2019/04/10-00:00:00&lt;/ACTION_DATE>
            &lt;TRANSACTION_ID>YY1651561&lt;/TRANSACTION_ID>
            &lt;CUSTOMER_ID>1288775&lt;/CUSTOMER_ID>
            &lt;USER_NAME>BSCS&lt;/USER_NAME>
            &lt;CHANNEL_NAME>BSCS&lt;/CHANNEL_NAME>
            &lt;MODIFY_INFO>
               &lt;CREATE_INFO>
                  &lt;CREATE_DATE>2019/04/10-00:00:00&lt;/CREATE_DATE>
                  &lt;CREATE_USER>BSCS&lt;/CREATE_USER>
               &lt;/CREATE_INFO>
            &lt;/MODIFY_INFO>
         &lt;/pReqInfo>
         &lt;pSubscInfo>
            &lt;SUBSCRIPTION_KEY>
               &lt;!--You have a CHOICE of the next 2 items at this level-->
               &lt;SUBSCRIPTION_ID>1633717&lt;/SUBSCRIPTION_ID>
            &lt;/SUBSCRIPTION_KEY>
            &lt;SUBSCRIPTION_DEFINITION>
               &lt;SUBSCRIPTION_NAME>${subName}&lt;/SUBSCRIPTION_NAME>
            &lt;/SUBSCRIPTION_DEFINITION>

            &lt;SUBSCRIPTION_CHAR_VAL>
               &lt;NAME>${subCharName}&lt;/NAME>
          &lt;END_DATE>2029/04/10-00:00:00&lt;/END_DATE>
            &lt;/SUBSCRIPTION_CHAR_VAL>
         &lt;/pSubscInfo>
      &lt;/aom:UpdateSubscription>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>UpdateSubscription</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.subCharName</defaultValue>
      <description></description>
      <id>c7076671-cf05-4038-b930-7867d361948c</id>
      <masked>false</masked>
      <name>subCharName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.subName</defaultValue>
      <description></description>
      <id>b95867e6-fa92-4df7-a1e6-428d48c2b88a</id>
      <masked>false</masked>
      <name>subName</name>
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
   <wsdlAddress>http://172.30.10.32:8181/AOMWSSubscription/AOMWSSubscription?wsdl</wsdlAddress>
</WebServiceRequestEntity>
