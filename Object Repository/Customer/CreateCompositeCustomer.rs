<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateCompositeCustomer</name>
   <tag></tag>
   <elementGuidId>b417bc8a-5424-468b-bfce-b0eb5bfdd6d5</elementGuidId>
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
      &lt;aom:CreateCompositeCustomer>
         &lt;pReqInfo>
            &lt;ACTION_INTERACTION_TYPE_ID>143&lt;/ACTION_INTERACTION_TYPE_ID>
            &lt;ACTION_DATE>2019/01/01-00:00:00&lt;/ACTION_DATE>
            &lt;TRANSACTION_ID>CA123456&lt;/TRANSACTION_ID>
            &lt;CUSTOMER_ID>990001001&lt;/CUSTOMER_ID>
            &lt;USER_NAME>cansuarslan&lt;/USER_NAME>
            &lt;CHANNEL_NAME>test&lt;/CHANNEL_NAME>
            &lt;SIMULATION_FLAG>false&lt;/SIMULATION_FLAG>
            &lt;MODIFY_INFO>
               &lt;CREATE_INFO>
                  &lt;CREATE_DATE>2017/12/17-10:00:00&lt;/CREATE_DATE>
                  &lt;CREATE_USER>cansuarslan&lt;/CREATE_USER>
               &lt;/CREATE_INFO>
            &lt;/MODIFY_INFO>
         &lt;/pReqInfo>
        &lt;pCustInfo>
            &lt;CUSTOMER_KEY>
               &lt;!--You have a CHOICE of the next 2 items at this level-->
               &lt;CUSTOMER_ID>${custID}&lt;/CUSTOMER_ID>
            &lt;/CUSTOMER_KEY>
            &lt;CUSTOMER_DEFINITION>
               &lt;CUSTOMER_NAME>cansuarslan&lt;/CUSTOMER_NAME>
               &lt;CUSTOMER_STATUS>A&lt;/CUSTOMER_STATUS>
               &lt;CUSTOMER_TYPE>I&lt;/CUSTOMER_TYPE>
               &lt;SEGMENT_CODE>123&lt;/SEGMENT_CODE>
            &lt;/CUSTOMER_DEFINITION>
         &lt;/pCustInfo>
        &lt;pBillAcct>
            &lt;BILLING_ACCOUNT_KEY>
               &lt;BILLING_ACCOUNT_ID>${billAcctID}&lt;/BILLING_ACCOUNT_ID>
            &lt;/BILLING_ACCOUNT_KEY>
            &lt;BILLING_ACCOUNT_DEFINITION>
               &lt;NAME_ON_INVOICE>Cansu Arslan&lt;/NAME_ON_INVOICE>
               &lt;PARENT_RELATION_ID>2&lt;/PARENT_RELATION_ID>
               &lt;ACCOUNT_TYPE>1&lt;/ACCOUNT_TYPE>
               &lt;ACCOUNT_STATUS>A&lt;/ACCOUNT_STATUS>
               &lt;PIN_CODE>3040&lt;/PIN_CODE>
               &lt;COMPANY_DEFINITION_ID>1&lt;/COMPANY_DEFINITION_ID>
               &lt;LATE_PAYMENT_EXEMPTION_FLAG>N&lt;/LATE_PAYMENT_EXEMPTION_FLAG>
               &lt;ACCOUNT_TAX_INFO>1&lt;/ACCOUNT_TAX_INFO>
            &lt;/BILLING_ACCOUNT_DEFINITION>
            &lt;BILLING_ACCOUNT_PROFILE>
               &lt;BILL_PRESENTATION_ID>2&lt;/BILL_PRESENTATION_ID>
               &lt;CREDIT_LIMIT>1000&lt;/CREDIT_LIMIT>
               &lt;CURRENCY_CODE>TRL&lt;/CURRENCY_CODE>
               &lt;FCBS_BILL_CYCLE_ID>1&lt;/FCBS_BILL_CYCLE_ID>
               &lt;CHARGING_TYPE>2&lt;/CHARGING_TYPE>
               &lt;BILL_ACCT_TYPE>1&lt;/BILL_ACCT_TYPE>
            &lt;/BILLING_ACCOUNT_PROFILE>
            &lt;BILLING_ACCOUNT_CONTACT>
               &lt;CONTACT_TYPE>P&lt;/CONTACT_TYPE>
               &lt;CONTACT_ADDRESS>
                  &lt;POSTAL_ADDRESS>
                     &lt;COUNTRY>TURKEY&lt;/COUNTRY>
                     &lt;CITY_NAME>ADANA&lt;/CITY_NAME>
                     &lt;SUB_PROVINCE_NAME>KOZAN&lt;/SUB_PROVINCE_NAME>
                     &lt;DISTRICT>MERKEZ&lt;/DISTRICT>
                     &lt;MAIN_STREET>KARACAOĞLAN&lt;/MAIN_STREET>
                     &lt;STREET>ADANA&lt;/STREET>
                     &lt;BUILDING>ARSLAN&lt;/BUILDING>
                     &lt;HOUSE_NUMBER>59&lt;/HOUSE_NUMBER>
                     &lt;FLOOR_NUMBER>2&lt;/FLOOR_NUMBER>
                     &lt;DOOR_NUMBER>2&lt;/DOOR_NUMBER>
                     &lt;POSTAL_CODE>01510&lt;/POSTAL_CODE>
                     &lt;DESCRIPTION>Hastane yanı Su deposu önü&lt;/DESCRIPTION>
                  &lt;/POSTAL_ADDRESS>
               &lt;/CONTACT_ADDRESS>
            &lt;/BILLING_ACCOUNT_CONTACT>
            &lt;BILLING_ACCOUNT_CHAR_VAL>
               &lt;NAME>cansuarslan&lt;/NAME>
            &lt;/BILLING_ACCOUNT_CHAR_VAL>
         &lt;/pBillAcct>
      &lt;/aom:CreateCompositeCustomer>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>CreateCompositeCustomer</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.custID</defaultValue>
      <description></description>
      <id>d99cc46c-cacf-403b-99be-f99ff1970873</id>
      <masked>false</masked>
      <name>custID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.billAcctID</defaultValue>
      <description></description>
      <id>0d92f7a7-d1a9-4dd5-a324-a637f909ffb4</id>
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



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getResponseText()).contains('Katalon Test Project')


//WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')

</verificationScript>
   <wsdlAddress>http://172.30.10.32:8181/AOMWSCustomer/AOMWSCustomer?wsdl</wsdlAddress>
</WebServiceRequestEntity>
