<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>InquireBillingAccount</name>
   <tag></tag>
   <elementGuidId>be8787b2-2643-47df-8c41-d6328fc98a79</elementGuidId>
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
      &lt;aom:InquireBillingAccount>
           &lt;pReqInfo>
            &lt;ACTION_INTERACTION_TYPE_ID>143&lt;/ACTION_INTERACTION_TYPE_ID>
            &lt;ACTION_DATE>2019/01/01-00:00:00&lt;/ACTION_DATE>
            &lt;TRANSACTION_ID>CA123456&lt;/TRANSACTION_ID>
            &lt;CUSTOMER_ID>${custID}&lt;/CUSTOMER_ID>
            &lt;USER_NAME>cansuarslan&lt;/USER_NAME>
            &lt;CHANNEL_NAME>test&lt;/CHANNEL_NAME>
            &lt;SIMULATION_FLAG>false&lt;/SIMULATION_FLAG>
            &lt;MODIFY_INFO>
               &lt;CREATE_INFO>
                  &lt;CREATE_DATE>2019/12/17-10:00:00&lt;/CREATE_DATE>
                  &lt;CREATE_USER>cansuarslan&lt;/CREATE_USER>
               &lt;/CREATE_INFO>
            &lt;/MODIFY_INFO>
         &lt;/pReqInfo>
         &lt;pKey>
            &lt;!--You have a CHOICE of the next 9 items at this level-->
            &lt;BILLING_ACCOUNT_ID>${billAcctID}&lt;/BILLING_ACCOUNT_ID>
         &lt;/pKey>
      &lt;/aom:InquireBillingAccount>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>InquireBillingAccount</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.custID</defaultValue>
      <description></description>
      <id>edb65720-7805-4f6e-a3fa-a2fff2a4515e</id>
      <masked>false</masked>
      <name>custID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.billAcctID</defaultValue>
      <description></description>
      <id>997481ed-4c29-41e1-90c4-92873adc35cb</id>
      <masked>false</masked>
      <name>billAcctID</name>
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
   <wsdlAddress>http://172.30.10.32:8181/AOMWSBillingAccount/AOMWSBillingAccount?wsdl</wsdlAddress>
</WebServiceRequestEntity>
